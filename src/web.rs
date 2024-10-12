use actix_web::{
    middleware,
    web::{get, resource, Data},
    App, HttpServer, Responder,
};
use askama::Template;

use crate::utils::{db::Database, discord::DiscordClient};

mod auth;
mod events;

async fn home_page() -> impl Responder {
    #[derive(Template)]
    #[template(path = "home.html")]
    struct HomePage;

    HomePage
}

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    let db = Database::new().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(DiscordClient::new()))
            .app_data(Data::new(reqwest::Client::new()))
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .service(actix_files::Files::new("/assets", "assets").use_last_modified(true))
            .service(resource("/").route(get().to(home_page)))
            .configure(auth::config)
            .configure(events::config)
    })
    .bind((host, port))?
    .run()
    .await
}
