#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let app = api::app(infra::BootstrapConfig {
        secret_manager_key: std::env::var("SECRET_CONFIG").ok(),
    })
    .await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
