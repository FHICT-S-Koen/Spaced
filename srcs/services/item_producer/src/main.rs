use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Router, Server};
use clap::Parser;
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use tracing_subscriber::{EnvFilter, filter::LevelFilter};
use anyhow::Result;

mod consumer;
mod handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long, default_value_t = 8080)]
  port: u16,

  #[arg(long, default_value_t = std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
  host: IpAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
  init_logging();

  let args = Args::parse();

  let (io_layer, io) = SocketIo::new_layer();
  io.ns("/", handlers::on_connection);
  tokio::spawn(consumer::background_task(io));

  let app = Router::new()
    .nest_service("/", ServeDir::new("dist"))
    .layer(
      ServiceBuilder::new()
        .layer(CorsLayer::permissive()) // Enable CORS policy
        .layer(io_layer),
    );

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
