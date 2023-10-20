#[macro_use]
extern crate log;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;

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

type LoggingConsumer = StreamConsumer<CustomContext>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  info!("Item consumer running");

  let (version_n, version_s) = get_rdkafka_version();
  info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
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

  loop {
    match consumer.recv().await {
      Err(e) => info!("Kafka error: {}", e),
      Ok(m) => {
        let payload = match m.payload_view::<str>() {
          None => "",
          Some(Ok(s)) => s,
          Some(Err(e)) => {
            info!("Error while deserializing message payload: {:?}", e);
            ""
          }
        };
        info!(
          "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
          m.key(),
          payload,
          m.topic(),
          m.partition(),
          m.offset(),
          m.timestamp()
        );
        if let Some(headers) = m.headers() {
          for header in headers.iter() {
            info!("  Header {:#?}: {:?}", header.key, header.value);
          }
        }
        consumer.commit_message(&m, CommitMode::Async).unwrap();
      }
    };
  }
}
