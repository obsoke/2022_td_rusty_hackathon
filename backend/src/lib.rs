use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, IntoMakeService},
    Json, Router,
};
use axum_extra::routing::SpaRouter;
use hyper::server::conn::AddrIncoming;
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::net::TcpListener;
use types::{Category, Flashcard};

// Hardcoding SQL DB URL for simplicity
const SQLITE_DB_URL: &'static str = "sqlite:cards.db";

pub type Server = hyper::server::Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn run(listener: TcpListener) -> Result<Server, hyper::Error> {
    tracing_subscriber::fmt::init();

    // set up sqlite & run all migrations
    let pool = create_sqlite(true).await.unwrap();

    // run app with hyper
    let server = axum::Server::from_tcp(listener)?.serve(app(pool).into_make_service());

    Ok(server)
}

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/user", get(get_user))
        .route("/api/card/:category", get(get_cards_for_category))
        .route("/api/card/:category", post(create_card_for_category))
        .route("/api/category", post(create_category))
        .route("/api/category", get(get_categories))
        .merge(SpaRouter::new("/assets", "../frontend/dist"))
        // Give all routes access to SQLite DB pool
        .layer(Extension(pool))
}

pub async fn create_sqlite(run_migrations: bool) -> Result<SqlitePool, ()> {
    let pool = SqlitePool::connect(SQLITE_DB_URL)
        .await
        .expect("Unable to connect to SQLite");

    if run_migrations {
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Unable to run migrations");
    }

    Ok(pool)
}

#[derive(Deserialize, serde::Serialize, Default, Clone, PartialEq)]
struct User {
    id: u64,
    username: String,
}

async fn get_user() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(User {
            id: 0,
            username: "Bob".to_string(),
        }),
    )
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

async fn get_cards_for_category(
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
struct CreateCardForCategory {
    question: String,
    answer: String,
}

async fn create_card_for_category(
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

#[derive(Deserialize)]
struct CreateCategory {
    name: String,
}

async fn create_category(
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

async fn get_categories(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
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
