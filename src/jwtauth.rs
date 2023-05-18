use crate::{Db, KEYS};

use std::fmt::Display;

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use edgedb_derive::Queryable;
use edgedb_protocol::model::Uuid;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongClientCredentials,
    WrongUserCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongClientCredentials => {
                (StatusCode::UNAUTHORIZED, "Wrong Client credentials")
            }
            AuthError::WrongUserCredentials => (StatusCode::UNAUTHORIZED, "Wrong User credentials"),
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

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Queryable, Debug, Deserialize)]
pub struct AuthPayload {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    admin: bool,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.username, self.admin)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        println!("{:?}", bearer.token());

        // Decode the user data
        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.decoding,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Queryable, Debug)]
pub struct Client {
    id: Uuid,
    secret: String,
}

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub struct User {
    name: String,
    email: String,
    password: String,
    salt: String,
}

pub async fn authorize(
    State(db): State<Db>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    println!("{:?}", payload);

    if payload.client_id.is_empty()
        || payload.client_secret.is_empty()
        || payload.username.is_empty()
        || payload.password.is_empty()
    {
        return Err(AuthError::MissingCredentials);
    }

    let client: Option<bool> = db
        .client
        .query_single(
            r#"
            with client := (select default::Client {
                secret
            } filter .id = <std::uuid>$0)
            select client.secret = <str>$1"#,
            &(payload.client_id, payload.client_secret),
        )
        .await
        .unwrap();

    let user: Option<bool> = db
        .client
        .query_single(
            r#"
            with user := select default::User {
                password,
            } filter .name = <str>$0
            select user.password = <str>$1"#,
            &(payload.username.clone(), payload.password),
        )
        .await
        .unwrap();

    println!("{:?}", client);
    println!("{:?}", user);

    let claims = Claims {
        username: payload.username,
        admin: true,
    };

    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthBody::new(token)))
}
