use axum::{
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use futures::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use serde::Serialize;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};

use crate::config::Config;
use crate::http_utils::init_tracing;

mod config;
mod http_utils;

type SharedState = Arc<AppState>;

struct AppState {
    received_ws_messages: RwLock<Vec<String>>,
    tx: broadcast::Sender<String>,
}

#[derive(Serialize)]
struct MessageList {
    messages: Vec<String>,
}

#[tokio::main]
async fn main() {
    init_tracing();
    let config = Config::from_env();
    let addr = config.socket_addr();
    info!("🚀 Service started on {}", &addr);
    info!("🔌 Socket listening at {}{}", &addr, &config.ws_path);

    let received_ws_messages = RwLock::new(vec![]);
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        received_ws_messages,
        tx,
    });

    let app = Router::new()
        .route("/messages", get(list_messages))
        .route("/messages", post(create_message))
        .route(&config.ws_path, get(ws_handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_messages(Extension(app_state): Extension<SharedState>) -> impl IntoResponse {
    let messages = app_state
        .received_ws_messages
        .read()
        .unwrap()
        .iter()
        .map(|msg| msg.to_string())
        .collect::<Vec<String>>();
    (StatusCode::OK, Json(messages))
}

async fn create_message(payload: String, app_state: Extension<SharedState>) -> impl IntoResponse {
    match app_state.tx.send(payload.clone()) {
        Ok(_) => debug!("generating mock websocket message: {}", payload),
        Err(_) => error!("failed generating websocket message"),
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(app_state): Extension<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, app_state))
}

async fn handle_socket(socket: WebSocket, app_state: SharedState) {
    info!("client upgraded to websocket");
    let (ws_sender, ws_receiver) = socket.split();

    tokio::spawn(read_from_ws(ws_receiver, app_state.clone()));
    tokio::spawn(write_to_ws(ws_sender, app_state));
}

async fn read_from_ws(mut receiver: SplitStream<WebSocket>, app_state: SharedState) {
    loop {
        if let Some(msg_result) = receiver.next().await {
            match msg_result {
                Ok(msg) => match msg {
                    WsMessage::Text(t) => {
                        app_state
                            .received_ws_messages
                            .write()
                            .unwrap()
                            .push(t.clone());
                        info!("client to server: {:?}", t);
                    }
                    WsMessage::Close(_) => {
                        info!("client disconnected");
                        return;
                    }
                    _ => info!("ignoring non-text message"),
                },
                Err(err) => error!("{}", err),
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn write_to_ws(mut sender: SplitSink<WebSocket, WsMessage>, app_state: SharedState) {
    let mut rx = app_state.tx.subscribe();
    loop {
        if let Ok(to_send) = rx.recv().await {
            sender
                .send(WsMessage::Text(to_send.clone()))
                .await
                .expect("deliver message");
            info!("server to client: {:?}", to_send);
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
