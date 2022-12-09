use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use types::Category;

#[derive(serde::Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub name: String,
}

#[tracing::instrument(
    name = "Creating a new category",
    skip(pool, payload),
    fields(
        request_id = %Uuid::new_v4(),
        category_name = %payload.name
    )
)]
pub async fn create_category(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateCategory>,
) -> impl IntoResponse {
    match insert_category(&pool, &payload).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(name = "Saving new category to the database", skip(pool, payload))]
pub async fn insert_category(
    pool: &SqlitePool,
    payload: &CreateCategory,
) -> Result<(), sqlx::Error> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    sqlx::query("INSERT INTO categories (name) VALUES (?)")
        .bind(&payload.name)
        .execute(&mut conn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to e");
            e
        })?;

    Ok(())
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
