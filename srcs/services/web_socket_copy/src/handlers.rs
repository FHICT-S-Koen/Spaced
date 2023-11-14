use amqprs::{
  callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
  channel::BasicPublishArguments,
  connection::{Connection, OpenConnectionArguments},
  BasicProperties,
};
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

pub async fn on_connection(socket: SocketRef) {
  info!("Connect AMQP producer");
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

  info!("Register handlers");

  socket.on("message", move |_s: SocketRef, Data::<String>(data)| {
    info!("Received event: {:?}", data);
    tokio::spawn(async move {
      let channel = connection.open_channel(None).await.unwrap();
      channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

      info!("Received event: {:?}", data);

      let routing_key = "amqprs.example";
      let exchange_name = "amq.topic";
      let args = BasicPublishArguments::new(exchange_name, routing_key);
      channel
        .basic_publish(BasicProperties::default(), data.into_bytes(), args)
        .await
        .unwrap();
    });
  });

  socket.on_disconnect(|socket, reason| async move {
    info!("Socket.IO disconnected: {} {}", socket.id, reason);
  });
}
