use assert_cmd::cargo::cargo_bin;
use std::{
    net::{SocketAddr, TcpStream},
    process::{Child, Command},
    thread,
};
use websocket::{ClientBuilder, Message};

#[test]
fn test_can_connect_to_ws() {
    let port = 3001;
    let process = start_service(port);
    let _test_control = TestControl::new(process);

    wait_ws_reachable(port);

    let connection = ClientBuilder::new(&format_url(port, "ws"))
        .unwrap()
        .connect_insecure();

    assert!(connection.is_ok());
}

#[test]
fn test_can_read_sent_messages() {
    let port = 3002;
    let process = start_service(port);
    let test_control = TestControl::new(process);

    wait_ws_reachable(port);

    let mut connection = ClientBuilder::new(&format_url(port, "ws"))
        .unwrap()
        .connect_insecure()
        .unwrap();

    let text = "hello from client";
    let message = Message::text(text);
    connection.send_message(&message).unwrap();

    let resp: Vec<String> = reqwest::blocking::get(format_url(port, "messages"))
        .expect("fetch messages")
        .json::<Vec<String>>()
        .expect("map messages");

    assert!(resp.contains(&String::from(text)));

    drop(test_control);
}

struct TestControl {
    service: Child,
}

impl TestControl {
    fn new(service: Child) -> Self {
        Self { service }
    }
}

impl Drop for TestControl {
    fn drop(&mut self) {
        self.service.kill().expect("Stop service process");
    }
}

fn format_url(port: u16, path: &str) -> String {
    format!("http://127.0.0.1:{}/{}", port, path)
}

fn start_service(port: u16) -> Child {
    let path = cargo_bin("webmocket");
    let mut command = Command::new(path);

    command.env("WEBMOCKET_PORT", port.to_string());
    command.spawn().expect("Spawn service process")
}

fn wait_ws_reachable(port: u16) {
    let timeout = std::time::Duration::new(10, 0);
    let addr = SocketAddr::new("127.0.0.1".parse().unwrap(), port);
    // The socketaddr connect_timeout will fail if the service hasn't started
    // at all. So we add a thread::sleep here.
    // TODO: Some sort of loop that reruns if connect() fails; essentially building our own
    // connect_timeout().
    thread::sleep(std::time::Duration::new(1, 0));
    TcpStream::connect_timeout(&addr, timeout).expect("Attempt to reach service");
}
