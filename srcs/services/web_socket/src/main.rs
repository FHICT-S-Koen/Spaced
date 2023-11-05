use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Router, Server};
use clap::Parser;
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(long, default_value_t = 8080)]
  port: u16,

  /// Name of the person to greet
  #[arg(long, default_value_t = std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))]
  host: IpAddr,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  tracing::subscriber::set_global_default(FmtSubscriber::default())?;

  let (io_layer, io) = SocketIo::new_layer();
  io.ns("/", handlers::on_connection);

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
