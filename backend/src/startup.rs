use axum::{
    routing::{get, post},
    {routing::IntoMakeService, Router},
};
use axum_extra::routing::SpaRouter;
use hyper::server::conn::AddrIncoming;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{config::Settings, routes::*};

pub type Server = hyper::server::Server<AddrIncoming, IntoMakeService<Router>>;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        configuration: Settings,
        sqlite_pool: Option<SqlitePool>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        //get_connection_pool(&configuration);
        let pool = if sqlite_pool.is_some() {
            sqlite_pool.unwrap()
        } else {
            get_connection_pool(&configuration)
        };
        if configuration.application.run_migrations {
            sqlx::migrate!()
                .run(&pool)
                .await
                .expect("Unable to run migrations");
        }

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;

        let port = listener.local_addr().unwrap().port();
        let server = run(listener, pool).await?;

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), hyper::Error> {
        self.server.await
    }
}

pub async fn run(listener: TcpListener, db_pool: SqlitePool) -> Result<Server, hyper::Error> {
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
        .with_state(pool)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub fn get_connection_pool(config: &Settings) -> SqlitePool {
    SqlitePoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(
            config
                .get_database_settings()
                .expect("Could not parse DB connection string"),
        )
}
