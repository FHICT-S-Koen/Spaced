use axum::{extract::State, Json, response::IntoResponse};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Item {
  id: i32,
  x: i32,
  y: i32,
  w: i32,
  h: i32,
  name: Option<String>,
  stylesheet: Option<String>,
  schema: Option<String>,
  user_id: i32,
}

#[derive(Deserialize)]
pub struct BoundingBoxPayload {
  xmin: i32,
  ymin: i32,
  xmax: i32,
  ymax: i32,
}

pub async fn get_nearby_items(
  State(pool): State<PgPool>,
  Json(payload): Json<BoundingBoxPayload>
) -> impl IntoResponse {
  let rows = sqlx::query_as!(
    Item,
    r#"
      SELECT *
      FROM item
      WHERE x BETWEEN $1 AND $3
        AND y BETWEEN $2 AND $4;
      "#,
    payload.xmin,
    payload.ymin,
    payload.xmax,
    payload.ymax
  )
  .fetch_all(&pool)
  .await.unwrap();

  Json(rows)
}
