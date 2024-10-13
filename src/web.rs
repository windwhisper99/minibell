use std::sync::Arc;

use actix_web::{
    middleware,
    web::{get, resource, Data},
    App, HttpServer, Responder,
};
use askama::Template;

use crate::{
    repos::{discord, member, session},
    services::auth::AuthService,
    utils::db::Database,
};

mod auth;
mod events;

async fn home_page() -> impl Responder {
    #[derive(Template)]
    #[template(path = "home.html")]
    struct HomePage;

    HomePage
}

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    let db = Arc::new(Database::new().await);
    let reqwest = Arc::new(reqwest::Client::new());

    let discord = Arc::new(discord::DiscordRepo::new(reqwest.clone()));
    let session = Arc::new(session::SessionRepo::new(db.clone()));
    let member = Arc::new(member::MemberRepo::new(db.clone()));

    let auth_service = AuthService::new(discord.clone(), member.clone(), session.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(Data::new(auth_service.clone()))
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
