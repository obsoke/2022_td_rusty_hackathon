use std::net::{SocketAddr, TcpListener};

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
                .uri("/health_checl")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
}

#[tokio::test]
async fn health_check_works_via_server() {
    // Arrange
    let pool = create_sqlite(true).await.unwrap();
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(pool).into_make_service())
            .await
            .unwrap();
    });
    let client = hyper::Client::new();

    // Act
    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{}/health_check", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    assert_eq!("0", response.headers()["Content-Length"].to_str().unwrap()); // This sucks.
}

async fn spawn_app() {
    backend::run().await.unwrap();
}
