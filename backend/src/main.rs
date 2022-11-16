use std::net::TcpListener;

use backend::config::get_configuration;
use backend::startup::run;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let config = get_configuration().expect("Failed to read config");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("Could not bind address for server");
    run(listener, &config).await?.await
}
