use std::net::TcpListener;

use axum::{body::Body, http::Request};
use backend::*;
use tower::ServiceExt;

#[tokio::test]
async fn health_check_works_via_app() {
    // Arrange
    let pool = create_sqlite(true).await.unwrap();
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
    let addr = spawn_app().await;
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
    assert_eq!("0", response.headers()["Content-Length"].to_str().unwrap()); // This sucks.
}

async fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port for testing");
    let port = listener.local_addr().unwrap().port();
    let server = backend::run(listener)
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
