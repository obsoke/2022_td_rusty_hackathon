use axum::{body::Body, http::Request};
use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use std::net::TcpListener;

use backend::{
    routes::{CreateCardForCategory, CreateCategory},
    startup::create_sqlite,
    telemetry::{get_subscriber, init_subscriber},
};

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

#[tokio::test]
async fn create_category_returns_a_200_for_valid_category() {
    // Arrange
    let test_app = spawn_app().await;
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
                .uri(format!("{}/api/category", test_app.address))
                .body(Body::from(serde_json::to_string(&category).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert
    assert!(response.status().is_success());
    // assert_eq!("0", response.headers()["Content-Length"].to_str().unwrap()); // This sucks.
    let created_category = sqlx::query!("SELECT * FROM categories",)
        .fetch_one(&test_app.db_pool)
        .await
        .expect("Failed to fetch created category");
    assert_eq!(created_category.name, category.name);
}

#[tokio::test]
async fn create_question_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrange
    let test_app = spawn_app().await;
    let client = hyper::Client::new();
    // Create test category
    let category = crate::CreateCategory {
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

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "dogki_test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub db_pool: SqlitePool,
}

async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port for testing");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let pool = create_sqlite("", true).await.unwrap();

    let server = backend::startup::run(listener, pool.clone())
        .await
        .expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: pool,
    }
}
