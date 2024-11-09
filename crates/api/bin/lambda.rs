use lambda_http::Error;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let config_key = std::env::var("CONFIG_KEY").expect("CONFIG_KEY must be set");

    let app = api::app(infra::BootstrapConfig {
        secret_manager_key: Some(config_key),
    })
    .await;
    lambda_http::run(app).await
}
