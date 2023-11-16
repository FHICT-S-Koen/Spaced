use amqprs::{
  callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
  channel::BasicPublishArguments,
  connection::{Connection, OpenConnectionArguments},
  BasicProperties,
};
use socketioxide::extract::{Data, SocketRef};
use tracing::info;

// pub mod item {
//   include!(concat!(env!("OUT_DIR"), "/item.rs"));
// }

// use item::ItemResponse;
// #[allow(unused_imports)] // Used to encode protobufs
// use prost::Message as ProtoMessage;

// use crate::item::ItemListResponse;

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

      let exchange_name = "amq.topic";
      let routing_key = "amqprs.example";
      let args = BasicPublishArguments::new(exchange_name, routing_key);
      // broadcast_item_updates(data)
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

// async fn broadcast_item_updates(item, ) {
//   channel
//     .basic_publish(BasicProperties::default(), data.into_bytes(), args)
//     .await
//     .unwrap();
// }
