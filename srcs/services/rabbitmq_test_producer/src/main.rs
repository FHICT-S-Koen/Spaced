#[macro_use]
extern crate log;

use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::BasicPublishArguments;
use amqprs::connection::{Connection, OpenConnectionArguments};
use amqprs::BasicProperties;
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

  let channel1 = connection.open_channel(None).await.unwrap();
  // channel1
  //   .register_callback(DefaultChannelCallback)
  //   .await
  //   .unwrap();
  let channel2 = connection.open_channel(None).await.unwrap();
  // channel2
  //   .register_callback(DefaultChannelCallback)
  //   .await
  //   .unwrap();

  let content = String::from(
    r#"
      {
        "publisher": "example"
        "data": "Hello, amqprs!"
      }
    "#,
  )
  .into_bytes();

  let routing_key = "amqprs.example";
  let exchange_name = "amq.topic";
  let args = BasicPublishArguments::new(exchange_name, routing_key);

  loop {
    channel1
      .basic_publish(BasicProperties::default(), content.clone(), args.clone())
      .await
      .unwrap();
    channel2
      .basic_publish(BasicProperties::default(), content.clone(), args.clone())
      .await
      .unwrap();
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
