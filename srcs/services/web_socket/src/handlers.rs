use std::sync::Arc;

use futures_util::StreamExt as _;
use redis::{AsyncCommands, Client};
use serde_json::Value;
use socketioxide::{Socket, adapter::LocalAdapter};
use tracing::info;

static REDIS_CHANNEL: &str = "redis";

pub async fn on_connection(socket: Arc<Socket<LocalAdapter>>, _auth: Value) {
  socket.on("message", |_, data: Value, bin, _| async move {
    info!("Received event: {:?} {:?}", data, bin);
    let client = Client::open("redis://127.0.0.1:6379/").expect("Failed to create Redis client");
    let mut connection = client
      .get_async_connection()
      .await
      .expect("Failed to establish Redis connection");
    connection
      .publish::<&str, String, ()>(REDIS_CHANNEL, data.to_string())
      .await
      .unwrap();
  });

  socket.on_disconnect(|socket, reason| async move {
    info!("Socket.IO disconnected: {} {}", socket.id, reason);
  });

  tokio::spawn(async move {
    let client = Client::open("redis://127.0.0.1:6379/").expect("Failed to create Redis client");
    let mut pubsub = client
      .get_async_connection()
      .await
      .expect("Failed to establish Redis connection")
      .into_pubsub();

    info!("Subscribing to redis {:?}", REDIS_CHANNEL);
    pubsub.subscribe(REDIS_CHANNEL).await.unwrap();

    let mut pubsub_stream = pubsub.on_message();
    loop {
      if let Some(msg) = pubsub_stream.next().await {
        let payload = msg.get_payload::<String>().unwrap();
        info!("Received Redis message");
        socket.emit("/", payload).unwrap();
      }
    }
  });
}
