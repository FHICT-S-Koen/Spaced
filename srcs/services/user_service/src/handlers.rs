use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::OsRng, SaltString}};
use axum::{extract::State, Json, http::StatusCode, response::{Response, IntoResponse}};
use once_cell::sync::Lazy;
use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

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

static KEYS: Lazy<Keys> = Lazy::new(|| {
  let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
  Keys::new(secret.as_bytes())
});

#[derive(sqlx::FromRow, Deserialize)]
struct User {
  id: i32,
  email: String,
  username: String,
  password: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  exp: usize,
}

#[derive(Debug)]
pub enum AuthError {
  WrongCredentials,
  MissingCredentials,
  TokenCreation,
  InvalidToken,
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

#[derive(Debug, Serialize)]
pub struct AuthBody {
  access_token: String,
  token_type: String,
}

impl AuthBody {
  fn new(access_token: String) -> Self {
    Self {
      access_token,
      token_type: "Bearer".to_string(),
    }
  }
}

#[derive(Deserialize)]
pub struct RegisterUserPayload {
  email: String,
  username: String,
  password: String,
}

pub async fn register_email(
  State(pool): State<PgPool>,
  Json(payload): Json<RegisterUserPayload>
) -> Result<Json<AuthBody>, AuthError>  {
  let email = payload.email.to_lowercase();
  let username = payload.username;
  let argon2 = Argon2::default();
  let salt = SaltString::generate(&mut OsRng);
  let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt).unwrap().to_string().into_bytes();
  // TODO: generate email verification code

  let mut transaction = pool.begin().await.unwrap();

  let user: User = sqlx::query_as!(
    User,
    r#"INSERT INTO "user" (id, email, username, password) VALUES (DEFAULT, $1, $2, $3) RETURNING *"#,
    email,
    username,
    password_hash
  )
  .fetch_one(&mut *transaction)
  .await.unwrap();

  transaction.commit().await.unwrap();

  let current_time = SystemTime::now();
  let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

  let claims = Claims {
    sub: user.email,
    exp: since_epoch.as_secs() as usize + 3600,
  };

  let token = encode(&Header::default(), &claims, &KEYS.encoding)
    .map_err(|_| AuthError::TokenCreation)?;

  Ok(Json(AuthBody::new(token)))
}
