use amqprs::{
  callbacks::DefaultConnectionCallback,
  channel::BasicPublishArguments,
  connection::{Connection, OpenConnectionArguments},
  BasicProperties,
};
use futures_util::{Future, StreamExt as _};
use socketioxide::extract::{SocketRef, Data};
use tracing::info;

pub async fn on_connection(socket: SocketRef) {
  let conn = Connection::open(&OpenConnectionArguments::new(
    "localhost",
    5672,
    "user",
    "bitnami",
  ))
  .await
  .unwrap();
  conn
    .register_callback(DefaultConnectionCallback)
    .await
    .unwrap();

  info!("CONNECTING");

  socket.on("message", |_socket: SocketRef, Data::<String>(data)| async move {
    info!("Received event: {:?}", data);
    let channel = conn.open_channel(None).await.unwrap();
    let routing_key = "amqprs.example";
    let exchange_name = "amq.topic";
    let args = BasicPublishArguments::new(exchange_name, routing_key);
    channel
      .basic_publish(BasicProperties::default(), data.into_bytes(), args)
      .await
      .unwrap();
  });

  socket.on_disconnect(|socket, reason| async move {
    info!("Socket.IO disconnected: {} {}", socket.id, reason);
  });

  // tokio::spawn(async move {
  //   let mut pubsub = REDIS_CLIENT
  //     .get_async_connection()
  //     .await
  //     .expect("Failed to establish Redis connection")
  //     .into_pubsub();

  //   info!("Subscribing to redis {:?}", REDIS_CHANNEL);
  //   pubsub.subscribe(REDIS_CHANNEL).await.unwrap();

  //   let mut pubsub_stream = pubsub.on_message();
  //   loop {
  //     if let Some(msg) = pubsub_stream.next().await {
  //       let payload = msg.get_payload::<String>().unwrap();
  //       info!("Received Redis message");
  //       socket.emit("/", payload).unwrap();
  //     }
  //   }
  // });
}
