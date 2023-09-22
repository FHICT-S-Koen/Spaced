// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};
use tauri::State;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let conn = PgPool::connect(&env::var("DATABASE_URL")?).await?;
  tauri::Builder::default()
    .manage(conn)
    .invoke_handler(tauri::generate_handler![select, insert_into, delete])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
struct Item {
  id: i32,
  x: Option<i32>,
  y: Option<i32>,
  text: Option<String>,
}

#[tauri::command]
async fn select(pool: State<'_, Pool<Postgres>>) -> Result<Vec<Item>, ()> {
  let rows = sqlx::query_as!(Item, "SELECT * FROM item")
    .fetch_all(&*pool)
    .await
    .unwrap();
  Ok(rows)
}

#[tauri::command]
async fn insert_into(
  pool: State<'_, Pool<Postgres>>,
  x: i32,
  y: i32,
  text: String,
) -> Result<Item, ()> {
  let item = sqlx::query_as!(
    Item,
    r#"
    INSERT INTO item ( x, y, text )
    VALUES ($1, $2, $3) RETURNING id, x, y, text;
    "#,
    x,
    y,
    text,
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
    WHERE id = $1
    "#,
    id,
  )
  .execute(&*pool)
  .await
  .unwrap();
  Ok(())
}
