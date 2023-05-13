mod tests;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};


#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/echo", get(echo_handler))
        .route("/api", get(root))
}

async fn root() -> &'static str {
    "Hello, Client!"
}

// A WebSocket handler that echos any message it receives.
async fn echo_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(echo_handle_socket)
}

// The actual echo logic.
async fn echo_handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(msg) = msg {
            if socket.send(Message::Text(format!("You said: {msg}")))
                .await.is_err(){
                break;
            }
        }
    }
}

