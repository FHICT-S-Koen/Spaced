#[macro_use]
extern crate log;

use std::time::Duration;

use rdkafka::Message;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, BaseConsumer};
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

use tonic::{transport::Server, Request, Response, Status};

use item::item_server::{Item, ItemServer};
use item::ItemListResponse;
use utils::BoundingBox;

#[allow(unused_imports)] // Used to encode protobufs
use prost::Message as ProtoMessage;

pub mod item {
  tonic::include_proto!("item");
}
pub mod utils {
  tonic::include_proto!("utils");
}

pub struct ItemMessanger {
  producer: FutureProducer,
  consumer: BaseConsumer,
}

#[tonic::async_trait]
impl Item for ItemMessanger {
  async fn nearby_items(
    &self,
    request: Request<BoundingBox>,
  ) -> Result<Response<ItemListResponse>, Status> {
    info!("Got a request from {:?}", request.remote_addr());

    let request = async move {
      let bounding = request.into_inner().encode_to_vec();
      let delivery_status = self.producer
        .send(
          FutureRecord::to("item")
            .key(&"bounding".to_string())
            .payload(bounding.as_slice())
            .headers(OwnedHeaders::new().insert(Header {
              key: "header_key",
              value: Some("header_value"),
            })),
          Duration::from_secs(0),
        )
        .await;
      info!("Delivery status for message {} received", "test");
      delivery_status
    };

    info!("Future completed. Result: {:?}", request.await);
    let response = async move {
      let value: ItemListResponse;
      loop {
        let message = self.consumer.poll(Duration::from_secs(1));
        match message {
          Some(response) => {
            value = ItemListResponse::decode(response.unwrap().payload().unwrap()).unwrap();
            break
          }
          None => continue,
        }
      }
      value
    };
    // info!("Future completed. Result: {:?}", response.await);

    Ok(Response::new(response.await))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  let addr = "127.0.0.1:50051".parse().unwrap();

  let producer: FutureProducer = ClientConfig::new()
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

  consumer
    .subscribe(&vec!["processed_item"])
    .expect("Can't subscribe to specified topics");

  let messanger = ItemMessanger {
    producer,
    consumer
  };
  info!("Item producer service listening on {}", addr);

  Server::builder()
    .add_service(ItemServer::new(messanger))
    .serve(addr)
    .await?;

  Ok(())
}
