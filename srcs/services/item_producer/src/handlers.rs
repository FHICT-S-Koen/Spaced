use std::sync::Arc;

use amqprs::{
  channel::{BasicPublishArguments, Channel},
  BasicProperties,
};
use socketioxide::extract::{AckSender, Data, SocketRef};
use tracing::info;

// pub mod item {
//   include!(concat!(env!("OUT_DIR"), "/item.rs"));
// }

// use item::ItemResponse;
// #[allow(unused_imports)] // Used to encode protobufs
// use prost::Message as ProtoMessage;

// use crate::item::ItemListResponse;

pub fn update(socket: SocketRef, Data(data): Data<String>, _ack: AckSender) {
  info!("Received event: {:?}", data);
  let channel = socket.extensions.get::<Arc<Channel>>().unwrap().clone();
  tokio::spawn(async move {
    let exchange_name = "amq.topic";
    let routing_key = "amqprs.example";
    let args = BasicPublishArguments::new(exchange_name, routing_key);
    // broadcast_item_updates(data)
    channel
      .basic_publish(BasicProperties::default(), data.into_bytes(), args)
      .await
      .unwrap();
  });
}

// async fn broadcast_item_updates(item, ) {
//   channel
//     .basic_publish(BasicProperties::default(), data.into_bytes(), args)
//     .await
//     .unwrap();
// }
