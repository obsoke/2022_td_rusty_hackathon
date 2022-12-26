use std::net::TcpListener;

use backend::config::get_configuration;
use backend::startup::{create_sqlite, run};
use backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let subscriber = get_subscriber("doki".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config");
    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("Could not bind address for server");

    let pool = create_sqlite(&config.database_connection_string, true)
        .await
        .expect("Failed to connect to Postgres");

    run(listener, pool).await?.await
}
