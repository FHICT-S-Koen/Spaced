use amqprs::{
  callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
  channel::{BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments},
  connection::{Connection, OpenConnectionArguments},
  consumer::AsyncConsumer,
  BasicProperties, Deliver,
};
use anyhow::{Result, Ok};
use async_trait::async_trait;
use socketioxide::SocketIo;
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
    info!("Consuming incoming message on channel: {}", channel.channel_id());
    self.socket.emit("/", _content).unwrap();
  }
}

pub async fn background_task(io: SocketIo) -> Result<()> {
  info!("Connect AMQP consumer");
  let connection = Connection::open(&OpenConnectionArguments::new(
    "localhost",
    5672,
    "user",
    "bitnami",
  ))
  .await?;
  connection
    .register_callback(DefaultConnectionCallback)
    .await?;

  let channel = connection.open_channel(None).await?;
  channel
    .register_callback(DefaultChannelCallback)
    .await?;

  let (queue_name, _, _) = channel
    .queue_declare(QueueDeclareArguments::durable_client_named(
      "amqprs.examples.basic",
    ))
    .await?.unwrap();

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
      .basic_consume(ItemConsumer::new(io), args)
      .await?;
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
  });
  let guard = Notify::new();
  guard.notified().await;
  Ok(())
}
