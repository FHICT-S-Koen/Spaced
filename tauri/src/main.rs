// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use sqlx::{Pool, Sqlite, SqlitePool};
use tauri::State;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let conn = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
  tauri::Builder::default()
    .manage(conn)
    .invoke_handler(tauri::generate_handler![select, insert_into, delete])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
struct Item {
  id: i64,
  x: Option<i64>,
  y: Option<i64>,
  text: Option<String>,
}

#[tauri::command]
async fn select(pool: State<'_, Pool<Sqlite>>) -> Result<Vec<Item>, ()>  {
  let rows = sqlx::query_as!(Item, "SELECT * FROM item").fetch_all(&*pool).await.unwrap();
  Ok(rows)
}

#[tauri::command]
async fn insert_into(pool: State<'_, Pool<Sqlite>>, x: i64, y: i64, text: String) -> Result<Item, ()>  {
  let user = sqlx::query_as!(
    Item,
    r#"
    INSERT INTO item ( x, y, text )
    VALUES (?1, ?2, ?3);

    SELECT * FROM item ORDER BY rowid DESC;
    "#, x, y, text,
  ).fetch_one(&*pool).await.unwrap();
  Ok(user)
}

#[tauri::command]
async fn delete(pool: State<'_, Pool<Sqlite>>, id: i64) -> Result<(), ()>  {
  sqlx::query!(
    r#"
    DELETE FROM item
    WHERE id = ?1
    "#, id,
  ).execute(&*pool).await.unwrap();
  Ok(())
}
