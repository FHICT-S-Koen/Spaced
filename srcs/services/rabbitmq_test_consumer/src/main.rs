#[macro_use]
extern crate log;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, BasicPublishArguments, QueueDeclareArguments, QueueBindArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::consumer::DefaultConsumer;
use amqprs::BasicProperties;
use tokio::sync::Notify;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_logging();
  info!("Item consumer running");

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
  let channel = connection.open_channel(None).await.unwrap();
  channel
    .register_callback(DefaultChannelCallback)
    .await
    .unwrap();
  let (queue_name, _, _) = channel
    .queue_declare(QueueDeclareArguments::durable_client_named(
      "amqprs.examples.basic",
    ))
    .await
    .unwrap()
    .unwrap();

  let routing_key = "amqprs.example";
  let exchange_name = "amq.topic";
  channel
    .queue_bind(QueueBindArguments::new(
      &queue_name,
      exchange_name,
      routing_key,
    ))
    .await
    .unwrap();


  let args = BasicConsumeArguments::new(&queue_name, "example_basic_pub_sub");

  channel
    .basic_consume(DefaultConsumer::new(args.no_ack), args)
    .await
    .unwrap();

    let guard = Notify::new();
    guard.notified().await;
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
