use backend::routes::{CreateCardForCategory, CreateCategory};
use hyper::{Body, Request};

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_question_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let test_app = spawn_app().await;
    let client = hyper::Client::new();
    let category = CreateCategory {
        name: "My Cool Category".to_string(),
    };
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
    assert!(response.status().is_success());

    let test_cases = vec![
        (
            CreateCardForCategory {
                question: "".to_string(),
                answer: "I am answer".to_string(),
            },
            "empty question",
        ),
        (
            CreateCardForCategory {
                question: "I am question".to_string(),
                answer: "".to_string(),
            },
            "empty answer",
        ),
    ];
    for (body, description) in test_cases {
        let response = client
            .request(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri(format!("{}/api/category/1", test_app.address))
                    .body(Body::from(serde_json::to_string(&body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            400,
            response.status(),
            "The API did not return a 400 Bad Request when the payload was {}.",
            description
        );
    }
}
