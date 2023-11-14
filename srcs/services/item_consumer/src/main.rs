#[macro_use]
extern crate log;

use std::time::Duration;

use rdkafka::client::ClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Header, Headers, Message, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use sqlx::{postgres::PgPool, Pool, Postgres};

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

#[derive(sqlx::FromRow, Debug)]
struct Item {
  id: i32,
  x: i32,
  y: i32,
  data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init();
  let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://admin:password@localhost:5432/spaced".to_string());
  let db_connection = PgPool::connect(&db_connection_str).await?;
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

  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");

  consumer
    .subscribe(&vec!["item"])
    .expect("Can't subscribe to specified topics");

  poll(consumer, &producer, db_connection).await;
  Ok(())
}

async fn fetch_items(
  pool: &Pool<Postgres>,
  xmin: i32,
  ymin: i32,
  xmax: i32,
  ymax: i32,
) -> anyhow::Result<Vec<Item>> {
  let rows = sqlx::query_as!(
    Item,
    r#"
      SELECT *
      FROM item
      WHERE x BETWEEN $1 AND $3
        AND y BETWEEN $2 AND $4;
      "#,
    xmin,
    ymin,
    xmax,
    ymax
  )
  .fetch_all(&*pool)
  .await?;
  Ok(rows)
}

#[allow(unused_imports)] // Used to encode protobufs
use prost::Message as ProtoMessage;

use crate::item::ItemListResponse;

pub mod utils {
  include!(concat!(env!("OUT_DIR"), "/utils.rs"));
}
pub mod item {
  include!(concat!(env!("OUT_DIR"), "/item.rs"));
}

async fn send(producer: &FutureProducer, items: item::ItemListResponse) {
  // let future = async move {
    let bounding = items.encode_to_vec();
    let _ = producer.send(
      FutureRecord::to("processed_item")
        .key(&"processed_item".to_string())
        .payload(bounding.as_slice())
        .headers(OwnedHeaders::new().insert(Header {
          key: "header_key",
          value: Some("header_value"),
        })),
      Duration::from_secs(0),
    ).await;
  // };
  // future.await;
}

impl ItemListResponse {
  fn from(items: Vec<Item>) -> ItemListResponse {
    ItemListResponse { item_response: items.into_iter().map(|i| ItemResponse {
      id: i.id,
      x: i.x,
      y: i.y,
      data: i.data,
    }).collect() }
  }
}

async fn poll(consumer: LoggingConsumer, producer: &FutureProducer, pool: Pool<Postgres>) {
  loop {
    match consumer.recv().await {
      Err(e) => info!("Kafka error: {}", e),
      Ok(m) => {
        let payload = utils::BoundingBox::decode(m.payload().unwrap()).unwrap();
        info!(
          "key: '{:?}', payload: '{:?}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
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
        let items = fetch_items(
          &pool,
          payload.xmin,
          payload.ymin,
          payload.xmax,
          payload.ymax,
        )
        .await
        .unwrap();
        info!("{:#?}", items.len());
        send(producer, ItemListResponse::from(items)).await;
        consumer.commit_message(&m, CommitMode::Async).unwrap();
      }
    };
  }
}
