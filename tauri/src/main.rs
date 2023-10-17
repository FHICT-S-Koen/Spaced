// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};
use tauri::State;

use item::item_client::ItemClient;
use item::{ItemRequest, ItemResponse};
use tokio::sync::Mutex;
use tonic::Response;
use tonic::transport::Channel;

pub mod item {
  tonic::include_proto!("item");
}

struct Client(Mutex<ItemClient<Channel>>);

impl ItemResponse {
  fn into(self) -> Item {
    return Item { id: self.id, x: self.x, y: self.y, color: Some(self.color), data: self.data }
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| "postgres://admin:password@localhost:5432/spaced".to_string());
  let conn = PgPool::connect(&db_connection_str).await?;
  let client = ItemClient::connect("http://127.0.0.1:50051").await?;

  tauri::Builder::default()
    .manage(conn)
    .manage(Client(client.into()))
    .invoke_handler(tauri::generate_handler![select, insert, update, delete, request])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[tauri::command]
async fn request(client: State<'_, Client>) -> Result<Item, ()> {
  let request = tonic::Request::new(ItemRequest {
    id: 0,
    x: 0,
    y: 0,
    color: "black".into(),
    data: "test".into(),
  });

  let response = client.0.lock().await.unary_item(request).await.unwrap();

  println!("RESPONSE={:?}", response);
  // let d = response.into_inner().into();
  // println!("RESPONSE={:?}", &d);

  Ok(response.into_inner().into())
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
struct Item {
  id: i32,
  x: i32,
  y: i32,
  color: Option<String>,
  data: String,
}

#[tauri::command]
async fn select(pool: State<'_, Pool<Postgres>>) -> Result<Vec<Item>, ()> {
  let rows = sqlx::query_as!(Item, "SELECT * FROM item;")
    .fetch_all(&*pool)
    .await
    .unwrap();
  Ok(rows)
}

#[tauri::command]
async fn insert(
  pool: State<'_, Pool<Postgres>>,
  x: i32,
  y: i32,
  color: Option<String>,
  data: String,
) -> Result<Item, ()> {
  let item = sqlx::query_as!(
    Item,
    r#"
    INSERT INTO item ( x, y, color, data )
    VALUES ($1, $2, $3, $4) RETURNING id, x, y, color, data;
    "#,
    x,
    y,
    color,
    data,
  )
  .fetch_one(&*pool)
  .await
  .unwrap();
  Ok(item)
}

#[tauri::command]
async fn update(
  pool: State<'_, Pool<Postgres>>,
  id: i32,
  x: i32,
  y: i32,
  color: Option<String>,
  data: String,
) -> Result<Item, ()> {
  let item = sqlx::query_as!(
    Item,
    r#"
    UPDATE item
    SET (x, y, color, data) = ($2, $3, $4, $5)
    WHERE id = $1
    RETURNING id, x, y, color, data;
    "#,
    id,
    x,
    y,
    color,
    data,
  )
  .fetch_one(&*pool)
  .await
  .unwrap();
  Ok(item)
}

#[tauri::command]
async fn delete(pool: State<'_, Pool<Postgres>>, id: i32) -> Result<(), ()> {
  sqlx::query!(
    r#"
    DELETE FROM item
    WHERE id = $1;
    "#,
    id,
  )
  .execute(&*pool)
  .await
  .unwrap();
  Ok(())
}
