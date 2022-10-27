use std::net::TcpListener;

use backend::run;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Could not bind address for server");
    run(listener).await?.await
}
