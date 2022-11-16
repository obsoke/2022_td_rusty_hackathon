use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

use types::Category;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub name: String,
}

pub async fn create_category(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateCategory>,
) -> impl IntoResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let category_id = sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind(payload.name)
        .execute(&mut conn)
        .await
        .expect("Unable to create new category")
        .last_insert_rowid();

    (StatusCode::CREATED, Json(category_id))
}

pub async fn get_categories(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let rows = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(rows))
}
