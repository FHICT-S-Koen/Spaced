use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, time::{SystemTime, UNIX_EPOCH}};

use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::OsRng, SaltString}};
use axum::{Router, Server, routing::{get, post}, extract::State, Json, http::StatusCode, response::{Response, IntoResponse}};
use clap::Parser;
use once_cell::sync::Lazy;
use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = 8081)]
  port: u16,

  #[arg(long, default_value_t = std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
  host: IpAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  tracing::subscriber::set_global_default(FmtSubscriber::default())?;

  let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://admin:password@localhost:5432/spaced".to_string());
  let db_connection = PgPool::connect(&db_connection_str).await?;

  let app = Router::new()
    .route("/item", post(get_nearby_items))
    .with_state(db_connection)
    .layer(CorsLayer::permissive()); // Enable CORS policy

  let addr = &SocketAddr::new(args.host, args.port);
  info!("Server starting on {}://{}", "http", addr);
  Server::bind(addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Item {
  id: i32,
  x: i32,
  y: i32,
  w: i32,
  h: i32,
  name: Option<String>,
  stylesheet: Option<String>,
  schema: Option<String>,
  user_id: i32,
}

#[derive(Deserialize)]
struct BoundingBoxPayload {
  xmin: i32,
  ymin: i32,
  xmax: i32,
  ymax: i32,
}

async fn get_nearby_items(
  State(pool): State<PgPool>,
  Json(payload): Json<BoundingBoxPayload>
) -> impl IntoResponse {
  let rows = sqlx::query_as!(
    Item,
    r#"
      SELECT *
      FROM item
      WHERE x BETWEEN $1 AND $3
        AND y BETWEEN $2 AND $4;
      "#,
    payload.xmin,
    payload.ymin,
    payload.xmax,
    payload.ymax
  )
  .fetch_all(&pool)
  .await.unwrap();

  Json(rows)
}
