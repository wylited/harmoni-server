pub mod tests;
pub mod db;
pub mod jwtauth;

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
    routing::{get, post},
    Router
};
use jwtauth::{Keys, authorize, AuthError, Claims};
use once_cell::sync::Lazy;
use tracing::info;

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Clone)]
pub struct Db {
    pub client: edgedb_tokio::Client
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await?;
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app().await).await?;
    Ok(())
}

async fn app() -> Router {
    let db = Db { client: edgedb_tokio::create_client().await.unwrap()};

    Router::new()
        .route("/echo", get(echo_handler))
        .route("/api", get(root))
        .route("/api/authorize", post(authorize))
        .route("/api/protected", get(protected))
        .with_state(db)
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
            if socket.send(Message::Text(format!("You said: {msg}"))).await.is_err(){
                break;
            }
        }
    }
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Depends on claims
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}
