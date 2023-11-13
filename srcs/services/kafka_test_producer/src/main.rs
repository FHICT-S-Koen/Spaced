#[macro_use]
extern crate log;

use std::time::Duration;

use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::message::{Header, Headers, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord, BaseProducer, BaseRecord};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logging();
  info!("Item consumer running");

  let producer: BaseProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  loop {
    producer
      .send(BaseRecord::to("item")
      .key(&"bounding".to_string())
      .payload("test".as_bytes())
      .headers(OwnedHeaders::new().insert(Header {
        key: "header_key",
        value: Some("header_value"),
      })));
      // .send(
      //   FutureRecord::to("item")
      //     .key(&"bounding".to_string())
      //     .payload("test".as_bytes())
      //     .headers(OwnedHeaders::new().insert(Header {
      //       key: "header_key",
      //       value: Some("header_value"),
      //     })),
      // )
      // .unwrap();
  }

  Ok(())
}

fn init_logging() {
  let env_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy();

  tracing_subscriber::fmt()
    .with_target(true)
    .with_level(true)
    .with_env_filter(env_filter)
    .init();
}
