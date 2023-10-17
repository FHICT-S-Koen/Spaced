use tonic::{transport::Server, Request, Response, Status};

use item::item_server::{Item, ItemServer};
use item::{ItemRequest, ItemResponse};

pub mod item {
  tonic::include_proto!("item");
}

#[derive(Default)]
pub struct ItemMessanger {}

#[tonic::async_trait]
impl Item for ItemMessanger {
  async fn unary_item(
    &self,
    request: Request<ItemRequest>,
  ) -> Result<Response<ItemResponse>, Status> {
    println!("Got a request from {:?}", request.remote_addr());

    let reply = ItemResponse {
      id: request.into_inner().id,
      x: 0,
      y: 0,
      color: "black".to_string(),
      data: "test".to_string(),
    };
    Ok(Response::new(reply))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "127.0.0.1:50051".parse().unwrap();
  let greeter = ItemMessanger::default();

  // println!("Item server listening on {}", addr);

  Server::builder()
    .add_service(ItemServer::new(greeter))
    .serve(addr)
    .await?;

  Ok(())
}
