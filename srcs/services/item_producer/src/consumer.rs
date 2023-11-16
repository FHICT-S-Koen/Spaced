use std::sync::Arc;

use amqprs::{
  callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
  channel::{BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments},
  connection::{Connection, OpenConnectionArguments},
  consumer::AsyncConsumer,
  BasicProperties, Deliver,
};
use anyhow::{Ok, Result};
use async_trait::async_trait;
use socketioxide::{SocketIo, extract::SocketRef};
use tokio::sync::Notify;
use tracing::info;

struct ItemConsumer {
  socket: SocketIo,
}

impl ItemConsumer {
  pub fn new(socket: SocketIo) -> Self {
    Self { socket }
  }
}

#[async_trait]
impl AsyncConsumer for ItemConsumer {
  async fn consume(
    &mut self,
    channel: &Channel,
    _deliver: Deliver,
    _basic_properties: BasicProperties,
    _content: Vec<u8>,
  ) {
    info!(
      "Consuming incoming message on channel: {}",
      channel.channel_id()
    );
    self.socket.emit("/", _content).unwrap();
  }
}

pub async fn background_task(socket: SocketIo, connection: Arc<Connection>) -> Result<()> {
  info!("Connect AMQP consumer");

  let channel = connection.open_channel(None).await?;
  channel.register_callback(DefaultChannelCallback).await?;

  let (queue_name, _, _) = channel
    .queue_declare(QueueDeclareArguments::default())
    .await?
    .unwrap();

  let exchange_name = "amq.topic";
  let routing_key = "amqprs.example";
  channel
    .queue_bind(QueueBindArguments::new(
      &queue_name,
      exchange_name,
      routing_key,
    ))
    .await?;

  let args = BasicConsumeArguments::new(&queue_name, "example_basic_pub_sub");

  tokio::spawn(async move {
    channel
      .basic_consume(ItemConsumer::new(socket), args)
      .await
      .unwrap();
    let guard = Notify::new();
    guard.notified().await;
  });
  let guard = Notify::new();
  guard.notified().await;
  Ok(())
}
