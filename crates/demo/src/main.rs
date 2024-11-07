use axum::{http::StatusCode, response::IntoResponse, routing::get, Json};
use lambda_http::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct Ping {
    message: &'static str,
}

pub async fn ping() -> impl IntoResponse {
    (
        StatusCode::IM_A_TEAPOT,
        Json(Ping {
            message: "Hello, World!",
        }),
    )
}

pub async fn demo() -> impl IntoResponse {
    (
        StatusCode::IM_A_TEAPOT,
        Json(Ping {
            message: "This is demo route",
        }),
    )
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let app = axum::Router::new()
        .route("/", get(ping))
        .route("/demo", get(demo));
    lambda_http::run(app).await
}
