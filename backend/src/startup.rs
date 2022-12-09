use axum::{
    routing::{get, post},
    {routing::IntoMakeService, Router},
};
use axum_extra::routing::SpaRouter;
use hyper::server::conn::AddrIncoming;
use sqlx::sqlite::SqlitePool;
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::routes::*;

pub type Server = hyper::server::Server<AddrIncoming, IntoMakeService<Router>>;

pub async fn run(listener: TcpListener, db_pool: SqlitePool) -> Result<Server, hyper::Error> {
    // run app with hyper
    let server = axum::Server::from_tcp(listener)?.serve(app(db_pool).into_make_service());

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
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn create_sqlite(
    connection_string: &str,
    run_migrations: bool,
) -> Result<SqlitePool, ()> {
    let pool = SqlitePool::connect(connection_string)
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
