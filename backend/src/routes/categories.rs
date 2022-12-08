use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

use types::Category;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub name: String,
}

pub async fn create_category(
    State(pool): State<SqlitePool>,
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

pub async fn get_categories(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect(r#"Could not acqure DB connection"#);

    match sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(&mut conn)
        .await
    {
        Ok(rows) => (StatusCode::OK, Json(rows)),
        Err(e) => {
            println!("Failed to execute query: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json::default())
        }
    }
}
