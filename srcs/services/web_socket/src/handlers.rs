use std::{sync::Arc, time::Duration};

use futures_util::StreamExt as _;
use rdkafka::{
  config::RDKafkaLogLevel,
  consumer::BaseConsumer,
  message::{Header, OwnedHeaders},
  producer::{BaseProducer, BaseRecord, FutureProducer, FutureRecord},
  ClientConfig, Message,
};
use redis::{AsyncCommands, Client};
use serde_json::Value;
use socketioxide::{adapter::LocalAdapter, Socket};
use tracing::info;

#[allow(unused_imports)] // Used to encode protobufs
use prost::Message as ProtoMessage;

// use crate::item::ItemListResponse;

pub mod utils {
  include!(concat!(env!("OUT_DIR"), "/utils.rs"));
}
pub mod item {
  include!(concat!(env!("OUT_DIR"), "/item.rs"));
}

static REDIS_CHANNEL: &str = "redis";

pub async fn on_connection(socket: Arc<Socket<LocalAdapter>>, _auth: Value) {
  let redis_client =
    Client::open("redis://127.0.0.1:6379/").expect("Failed to create Redis client");

  let producer: BaseProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  let consumer: BaseConsumer = ClientConfig::new()
    .set("group.id", "example_producer_group_id")
    .set("bootstrap.servers", "localhost:9092")
    .set("enable.partition.eof", "false")
    .set("session.timeout.ms", "6000")
    .set("enable.auto.commit", "true")
    .set_log_level(RDKafkaLogLevel::Debug)
    .create()
    .expect("Consumer creation failed");

  socket.extensions.insert(redis_client);
  socket.extensions.insert(producer);
  socket.extensions.insert(consumer);

  socket.on("message", |socket, data: Value, bin, _| async move {
    info!("Received event: {:?} {:?}", data, bin);
    let client = socket.extensions.get::<Client>().unwrap().clone();
    let mut connection = client
      .get_async_connection()
      .await
      .expect("Failed to establish Redis connection");
    connection
      .publish::<&str, String, ()>(REDIS_CHANNEL, data.to_string())
      .await
      .unwrap();
  });

  socket.on("get:items", |socket, _: Value, bin, ack| async move {
    // let d = utils::BoundingBox::encode(bin);
    let f = bin.into_iter().flatten().collect::<Vec<u8>>();
    let producer = socket.extensions.get::<BaseProducer>().unwrap();
    let request = {
      let delivery_status = producer.send(
        BaseRecord::to("item")
          .key("bounding")
          .payload(&f)
          .headers(OwnedHeaders::new().insert(Header {
            key: "header_key",
            value: Some("header_value"),
          })),
      );
      info!("Delivery status for message {} received", "test");
      delivery_status
    };
    info!("Future completed. Result: {:?}", request);

    let consumer = socket.extensions.get::<BaseConsumer>().unwrap();
    let response = loop {
      if let Some(msg) = consumer.poll(Duration::from_secs(1)) {
        break msg.unwrap();
      }
    };
    ack.send(response.payload().unwrap()).ok();
  });

  socket.on_disconnect(|socket, reason| async move {
    info!("Socket.IO disconnected: {} {}", socket.id, reason);
  });

  tokio::spawn(async move {
    let client = socket.extensions.get::<Client>().unwrap().clone();
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
