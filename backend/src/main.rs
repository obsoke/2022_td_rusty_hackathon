use backend::config::get_configuration;
use backend::startup::Application;
use backend::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber("doki".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config");
    let application = Application::build(config, None).await?;
    application.run_until_stopped().await?;

    Ok(())
}
