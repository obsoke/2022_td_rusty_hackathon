use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;

use types::Flashcard;

pub async fn get_cards_for_category(
    Extension(pool): Extension<SqlitePool>,
    Path(category): Path<i32>,
) -> impl IntoResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let rows = sqlx::query_as::<_, Flashcard>("SELECT * FROM cards WHERE category = ?")
        .bind(category)
        .fetch_all(&mut conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(rows))
}

#[derive(Deserialize)]
pub struct CreateCardForCategory {
    question: String,
    answer: String,
}

pub async fn create_card_for_category(
    Extension(pool): Extension<SqlitePool>,
    Path(category): Path<i32>,
    Json(payload): Json<CreateCardForCategory>,
) -> impl IntoResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let card_id = sqlx::query("INSERT INTO cards (question, answer, category) VALUES (?, ?, ?)")
        .bind(payload.question)
        .bind(payload.answer)
        .bind(category)
        .execute(&mut conn)
        .await
        .expect("Unable to create new category")
        .last_insert_rowid();

    (StatusCode::CREATED, Json(card_id))
}
