mod tests;

use std::fmt::Display;

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade, FromRequestParts,
    },
    http::{request::Parts, StatusCode},
    response::{Response, IntoResponse},
    routing::{get, post},
    Router, Json, async_trait,
    RequestPartsExt
};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{EncodingKey, DecodingKey, Header, encode, Validation, decode};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

#[derive(Debug)]
enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    email: String,
    admin: bool,
}


impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.username, self.email, self.admin)
    }
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        println!("parts: {:?}", parts);

        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        // Decode the user data
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

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

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() || payload.username.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // replace with proper db call later
    if payload.client_id != "cid" || payload.client_secret != "csecret" || payload.username != "bob" || payload.password != "pass" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        username: payload.username,
        email: "bob@123.com".to_string(),
        admin: true,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}
