use std::sync::Arc;

use actix_web::{
    middleware,
    web::{scope, Data},
    App, HttpServer,
};

use crate::infra;

mod auth;
// mod events;
mod dto;
mod utils;

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    let db = Arc::new(infra::Database::new().await);
    let reqwest = Arc::new(reqwest::Client::new());

    let discord_req = infra::DiscordReq::new(reqwest.clone());
    let session_hmac = infra::SessionHmac::new();

    let member_repo = infra::MemberRepo::new(db.clone());
    let session_repo = infra::SessionRepo::new(db.clone());
    let event_repo = infra::EventRepo::new(db.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(discord_req.clone()))
            .app_data(Data::new(session_hmac.clone()))
            .app_data(Data::new(member_repo.clone()))
            .app_data(Data::new(session_repo.clone()))
            .app_data(Data::new(event_repo.clone()))
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .service(scope("api").configure(auth::config))
        // .configure(events::config)
    })
    .bind((host, port))?
    .run()
    .await
}
