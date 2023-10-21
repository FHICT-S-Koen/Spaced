#[macro_use]
extern crate log;

use std::time::Duration;

use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

use tonic::{transport::Server, Request, Response, Status};

use item::item_server::{Item, ItemServer};
use item::{ItemRequest, ItemResponse};
use bounding::BoundingBox;

use prost::Message as ProtoMessage;
pub mod item {
  tonic::include_proto!("item");
}
pub mod bounding {
  tonic::include_proto!("bounding");
}

#[derive(Default)]
pub struct ItemMessanger {}

#[tonic::async_trait]
impl Item for ItemMessanger {
  async fn unary_item(
    &self,
    request: Request<ItemRequest>,
  ) -> Result<Response<ItemResponse>, Status> {
    info!("Got a request from {:?}", request.remote_addr());

    let reply = ItemResponse {
      id: request.into_inner().id,
      x: 0,
      y: 0,
      color: "black".to_string(),
      data: "test".to_string(),
    };

    let producer: &FutureProducer = &ClientConfig::new()
      .set("bootstrap.servers", "localhost:9092")
      .set("message.timeout.ms", "5000")
      .create()
      .expect("Producer creation error");

    let future = async move {
      let bounding = BoundingBox {
        xmin: 0,
        ymin: 0,
        xmax: 0,
        ymax: 0,
      }.encode_to_vec();
      let delivery_status = producer
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

    info!("Future completed. Result: {:?}", future.await);

    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  let addr = "127.0.0.1:50051".parse().unwrap();
  let messanger = ItemMessanger::default();

  info!("Item producer service listening on {}", addr);

  Server::builder()
    .add_service(ItemServer::new(messanger))
    .serve(addr)
    .await?;

  Ok(())
}
