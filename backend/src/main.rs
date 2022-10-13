use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension as CreateExtension, Json, Router,
};
use axum_extra::routing::SpaRouter;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use types::{Category, Flashcard};

// Hardcoding SQL DB URL for simplicity
const SQLITE_DB_URL: &'static str = "sqlite:cards.db";

#[derive(Clone)]
struct Context {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // set up sqlite & run all migrations
    let pool = SqlitePool::connect(SQLITE_DB_URL)
        .await
        .expect("Unable to connect to SQLite");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Unable to run migrations");

    // create axum app / router
    let app = Router::new()
        .route("/api/card/:category", get(get_cards_for_category))
        .route("/api/card/:category", post(create_card_for_category))
        .route("/api/category", post(create_category))
        .route("/api/category", get(get_categories))
        .merge(SpaRouter::new("/assets", "../frontend/dist"))
        // Give all routes access to SQLite DB pool
        .layer(ServiceBuilder::new().layer(CreateExtension(Context { db: pool })));

    // run app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start Axum server");
}

async fn get_cards_for_category(
    ctx: Extension<Context>,
    Path(category): Path<i32>,
) -> impl IntoResponse {
    let mut conn = ctx
        .db
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
struct CreateCardForCategory {
    question: String,
    answer: String,
}

async fn create_card_for_category(
    ctx: Extension<Context>,
    Path(category): Path<i32>,
    Json(payload): Json<CreateCardForCategory>,
) -> impl IntoResponse {
    let mut conn = ctx
        .db
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

#[derive(Deserialize)]
struct CreateCategory {
    name: String,
}

async fn create_category(
    ctx: Extension<Context>,
    Json(payload): Json<CreateCategory>,
) -> impl IntoResponse {
    let mut conn = ctx
        .db
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

async fn get_categories(ctx: Extension<Context>) -> impl IntoResponse {
    let mut conn = ctx
        .db
        .acquire()
        .await
        .expect("Could not acqure DB connection");

    let rows = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(&mut conn)
        .await
        .unwrap();

    (StatusCode::OK, Json(rows))
}
