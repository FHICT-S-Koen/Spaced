use amqprs::{
  callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
  channel::{BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments},
  connection::{Connection, OpenConnectionArguments},
  consumer::AsyncConsumer,
  BasicProperties, Deliver,
};
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
    _channel: &Channel,
    _deliver: Deliver,
    _basic_properties: BasicProperties,
    _content: Vec<u8>,
  ) {
    self.socket.emit("/", _content).unwrap();
  }
}

pub async fn background_task(io: SocketIo) {
  info!("Connect AMQP consumer");
  let connection = Connection::open(&OpenConnectionArguments::new(
    "localhost",
    5672,
    "user",
    "bitnami",
  ))
  .await
  .unwrap();
  connection
    .register_callback(DefaultConnectionCallback)
    .await
    .unwrap();

  let channel = connection.open_channel(None).await.unwrap();
  channel
    .register_callback(DefaultChannelCallback)
    .await
    .unwrap();

  let (queue_name, _, _) = channel
    .queue_declare(QueueDeclareArguments::durable_client_named(
      "amqprs.examples.basic",
    ))
    .await
    .unwrap()
    .unwrap();

  let routing_key = "amqprs.example";
  let exchange_name = "amq.topic";
  channel
    .queue_bind(QueueBindArguments::new(
      &queue_name,
      exchange_name,
      routing_key,
    ))
    .await
    .unwrap();

  let args = BasicConsumeArguments::new(&queue_name, "example_basic_pub_sub");

  tokio::spawn(async move {
    channel
      .basic_consume(ItemConsumer::new(io), args)
      .await
      .unwrap();
    let guard = Notify::new();
    guard.notified().await;
  });
  let guard = Notify::new();
  guard.notified().await;
}
