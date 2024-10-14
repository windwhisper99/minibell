use std::sync::Arc;

use actix_web::{
    middleware,
    web::{get, resource, Data},
    App, HttpServer, Responder,
};
use askama::Template;
use utils::templates;

use crate::{
    domain::{auth::AccessType, Error},
    infra::{self, DiscordReq},
};

mod auth;
mod events;
mod utils;

async fn home_page(
    access_type: AccessType,
    discord_req: Data<DiscordReq>,
) -> Result<impl Responder, Error> {
    #[derive(Template)]
    #[template(path = "home.html", escape = "none")]
    struct HomePage {
        user_status: templates::UserStatusTempl,
    }

    Ok(HomePage {
        user_status: templates::UserStatusTempl::new(&access_type, &discord_req),
    })
}

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    // let db = Arc::new(Database::new().await);
    let db = Arc::new(infra::Database::new().await);
    let reqwest = Arc::new(reqwest::Client::new());

    let discord_req = infra::DiscordReq::new(reqwest.clone());
    let session_hmac = infra::SessionHmac::new();

    let member_repo = infra::MemberRepo::new(db.clone());
    let session_repo = infra::SessionRepo::new(db.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(discord_req.clone()))
            .app_data(Data::new(session_hmac.clone()))
            .app_data(Data::new(member_repo.clone()))
            .app_data(Data::new(session_repo.clone()))
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
