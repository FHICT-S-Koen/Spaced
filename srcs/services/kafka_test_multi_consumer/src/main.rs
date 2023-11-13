#[macro_use]
extern crate log;

use std::time::Duration;

use futures_util::StreamExt;
use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, BaseConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, Headers, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::Timeout;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
  fn pre_rebalance(&self, rebalance: &Rebalance) {
    info!("Pre rebalance {:?}", rebalance);
  }

  fn post_rebalance(&self, rebalance: &Rebalance) {
    info!("Post rebalance {:?}", rebalance);
  }

  fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
    info!("Committing offsets: {:?}", result);
  }
}

type LoggingConsumer = BaseConsumer<CustomContext>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logging();

  info!("Item consumer running");

  let context = CustomContext;
  let consumer: LoggingConsumer = ClientConfig::new()
    .set("group.id", "example_consumer_group_id")
    .set("bootstrap.servers", "localhost:9092")
    .set("enable.partition.eof", "false")
    .set("session.timeout.ms", "6000")
    .set("enable.auto.commit", "true")
    .set_log_level(RDKafkaLogLevel::Debug)
    .create_with_context(context)
    .expect("Consumer creation failed");

  consumer
    .subscribe(&vec!["item"])
    .expect("Can't subscribe to specified topics");

  // tokio::spawn(async move {
  let mut num = 0;
  loop {
    match consumer.poll(Timeout::Never) {
      Some(m) => {
        num += 1;
        info!("{:?} count: {}", m, num)
      }
      None => todo!(),
    }
  }
  // });

  // loop {

  // }

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
