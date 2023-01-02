use backend::routes::CreateCategory;
use hyper::{Body, Request};

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_category_returns_a_200_for_valid_category() {
    // Arrange
    let test_app = spawn_app().await;
    let client = hyper::Client::new();

    let category = CreateCategory {
        name: "My Cool Category".to_string(),
    };

    // Act
    let response = client
        .request(
            Request::builder()
                .method("POST")
                .header("Content-Type", "application/json")
                .uri(format!("{}/api/category", test_app.address))
                .body(Body::from(serde_json::to_string(&category).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());

    let created_category = sqlx::query!("SELECT * FROM categories")
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch created category");
    assert_eq!(created_category.name, category.name);
}
