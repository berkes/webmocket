use assert_cmd::prelude::*;
use std::{
    net::{SocketAddr, TcpStream},
    process::{Child, Command},
    thread,
};
use websocket::ClientBuilder;

// Hack to enforce cleanup after tests.
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

#[test]
fn test_can_connect_to_ws() {
    let process = start_service();
    let _test_control = TestControl::new(process);

    wait_ws_reachable();

    let connection = ClientBuilder::new("http://127.0.0.1:3000/ws")
        .unwrap()
        .connect_insecure();

    assert!(connection.is_ok());
}

fn start_service() -> Child {
    let mut command = Command::cargo_bin("webmocket").expect("Prepare cargo bin command");
    command.spawn().expect("Spawn service process")
}

fn wait_ws_reachable() {
    let timeout = std::time::Duration::new(10, 0);
    let addr: SocketAddr = "127.0.0.1:3000".parse().expect("Parse socket address");
    // The socketaddr connect_timeout will fail if the service hasn't started
    // at all. So we add a thread::sleep here.
    // TODO: Some sort of loop that reruns if connect() fails; essentially building our own
    // connect_timeout().
    thread::sleep(std::time::Duration::new(1, 0));
    TcpStream::connect_timeout(&addr, timeout).expect("Attempt to reach service");
}
