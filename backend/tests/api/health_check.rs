use hyper::{Body, Request};

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works_via_server() {
    // Arrange
    let test_app = spawn_app().await;
    let client = hyper::Client::new();

    // Act
    let response = client
        .request(
            Request::builder()
                // .uri(format!("http://{}/api/health_check", addr))
                .uri(format!("{}/api/health_check", test_app.address))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
}
