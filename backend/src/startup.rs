use axum::{
    extract::Extension,
    routing::{get, post},
    {routing::IntoMakeService, Router},
};
use axum_extra::routing::SpaRouter;
use hyper::server::conn::AddrIncoming;
use sqlx::sqlite::SqlitePool;
use std::net::TcpListener;

use crate::{config::Settings, routes::*};

const SQLITE_DB_URL: &'static str = "sqlite:cards.db";
pub type Server = hyper::server::Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn run(listener: TcpListener, settings: &Settings) -> Result<Server, hyper::Error> {
    // tracing_subscriber::fmt::init(); // TODO: Re-enable when it doesn't cause issues in tests

    // set up sqlite & run all migrations
    let pool = create_sqlite(Some(&settings.database_connection_string), true)
        .await
        .unwrap();

    // run app with hyper
    let server = axum::Server::from_tcp(listener)?.serve(app(pool).into_make_service());

    Ok(server)
}

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/health_check", get(health_check))
        .route("/api/card/:category", get(get_cards_for_category))
        .route("/api/category/:category", post(create_card_for_category))
        .route("/api/category", post(create_category))
        .route("/api/category", get(get_categories))
        .merge(SpaRouter::new("/assets", "../frontend/dist"))
        // Give all routes access to SQLite DB pool
        .layer(Extension(pool))
}

pub async fn create_sqlite(
    connection_string: Option<&str>,
    run_migrations: bool,
) -> Result<SqlitePool, ()> {
    let conn_str = if connection_string.is_some() {
        connection_string.unwrap()
    } else {
        SQLITE_DB_URL
    };
    let pool = SqlitePool::connect(conn_str)
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
