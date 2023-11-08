use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Router, Server, routing::post};
use clap::Parser;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, filter::LevelFilter};

mod handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = 8082)]
  port: u16,

  #[arg(long, default_value_t = std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
  host: IpAddr,

  #[arg(long, default_value_t = String::from("postgres://admin:password@localhost:5432/spaced"))]
  database_host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logging();

  let args = Args::parse();

  info!("Connecting to database host {}", args.database_host);
  let db_connection = PgPool::connect(&args.database_host).await?;

  let app = Router::new()
    .route("/register", post(handlers::register_email))
    .with_state(db_connection)
    .layer(CorsLayer::permissive()); // Enable CORS policy

  let addr = &SocketAddr::new(args.host, args.port);
  info!("Server starting on {}://{}", "http", addr);
  Server::bind(addr)
    .serve(app.into_make_service())
    .await?;

  Ok(())
}

fn init_logging() {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();

  tracing_subscriber::fmt()
    .with_target(true)
    .with_level(true)
    .with_env_filter(env_filter)
    .init();
}
