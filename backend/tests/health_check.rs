use axum::{body::Body, http::Request};
use sqlx::{Connection, SqliteConnection};
use std::net::TcpListener;
use tower::util::ServiceExt;

use backend::{
    config::{get_configuration, Settings},
    routes::CreateCategory,
    startup::{app, create_sqlite},
};

#[tokio::test]
async fn health_check_works_via_app() {
    // Arrange
    let pool = create_sqlite(None, true).await.unwrap();
    let app = app(pool);

    // Act
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health_check")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    assert_eq!("0", response.headers()["Content-Length"].to_str().unwrap()); // This sucks.
}

#[tokio::test]
async fn health_check_works_via_server() {
    // Arrange
    let config = get_configuration().expect("Failed to read config");
    let addr = spawn_app(&config).await;
    let client = hyper::Client::new();

    // Act
    let response = client
        .request(
            Request::builder()
                // .uri(format!("http://{}/api/health_check", addr))
                .uri(format!("{}/api/health_check", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn create_category_returns_a_200_for_valid_category() {
    // Arrange
    let config = get_configuration().expect("Failed to read config");
    let addr = spawn_app(&config).await;
    let connection_string = config.database_connection_string;
    let _connection = SqliteConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to SQLlite");
    let client = hyper::Client::new();
    let category = crate::CreateCategory {
        name: "My Cool Category".to_string(),
    };

    // Act
    let response = client
        .request(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .uri(format!("{}/api/category", addr))
                .body(Body::from(serde_json::to_string(&category).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    println!("{response:?}");
    assert!(response.status().is_success());
    // assert_eq!("0", response.headers()["Content-Length"].to_str().unwrap()); // This sucks.
}

async fn spawn_app(config: &Settings) -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port for testing");
    let port = listener.local_addr().unwrap().port();
    let server = backend::startup::run(listener, config)
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
