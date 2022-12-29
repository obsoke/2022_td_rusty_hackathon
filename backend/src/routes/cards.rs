use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

use crate::domain::NewCardForCategory;
use types::Flashcard;

pub async fn get_cards_for_category(
    State(pool): State<SqlitePool>,
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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateCardForCategory {
    pub question: String,
    pub answer: String,
}

impl TryFrom<CreateCardForCategory> for NewCardForCategory {
    type Error = String;

    fn try_from(value: CreateCardForCategory) -> Result<Self, Self::Error> {
        let question = crate::domain::CardQuestion::parse(&value.question)?;
        let answer = crate::domain::CardAnswer::parse(&value.answer)?;
        Ok(NewCardForCategory { question, answer })
    }
}

#[tracing::instrument(
    name = "Creating a new card for a category",
    skip(pool, category, payload),
    fields(
        request_id = %Uuid::new_v4(),
        category_id = %category,
        question = %payload.question,
        answer = %payload.answer,
    )
)]
pub async fn create_card_for_category(
    State(pool): State<SqlitePool>,
    Path(category): Path<i32>,
    Json(payload): Json<CreateCardForCategory>,
) -> impl IntoResponse {
    let new_card = match payload.try_into() {
        Ok(card) => card,
        Err(_) => return StatusCode::BAD_REQUEST,
    };
    match insert_card(&pool, category, &new_card).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[tracing::instrument(
    name = "Saving new card to the database",
    skip(pool, category, payload)
)]
pub async fn insert_card(
    pool: &SqlitePool,
    category: i32,
    payload: &NewCardForCategory,
) -> Result<i64, sqlx::Error> {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let row_id = sqlx::query("INSERT INTO cards (question, answer, category) VALUES (?, ?, ?)")
        .bind(&payload.question.as_ref())
        .bind(&payload.answer.as_ref())
        .bind(category)
        .execute(&mut conn)
        .await
        .map_err(|e| {
            tracing::error!("Failed to e");
            e
        })?
        .last_insert_rowid();

    Ok(row_id)
}

pub fn parse_card(payload: &CreateCardForCategory) -> Result<NewCardForCategory, String> {
    let question = crate::domain::CardQuestion::parse(&payload.question)?;
    let answer = crate::domain::CardAnswer::parse(&payload.answer)?;
    Ok(NewCardForCategory { question, answer })
}
