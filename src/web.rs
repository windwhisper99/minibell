use std::{future::Future, pin::Pin, sync::Arc};

use actix_web::{
    middleware,
    web::{get, resource, Data},
    App, FromRequest, HttpServer, Responder,
};
use askama::Template;

use crate::{domain::auth::AccessType, infra, usecase};

mod auth;
mod events;

impl FromRequest for AccessType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req.cookie("token").map(|cookie| cookie.value().to_string());
        let session_hmac = req
            .app_data::<Data<infra::SessionHmac>>()
            .expect("SessionHmac not found")
            .clone();
        let session_repo = req
            .app_data::<Data<infra::SessionRepo>>()
            .expect("SessionRepo not found")
            .clone();

        Box::pin(async move {
            match token {
                Some(token) => usecase::verify_auth::VerifyUC::new(
                    session_repo.as_ref(),
                    session_hmac.as_ref(),
                )
                .execute(&token)
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Verify error")),
                _ => Ok(AccessType::Unauthenticated),
            }
        })
    }
}

async fn home_page(auth: AccessType) -> impl Responder {
    println!("{:?}", auth);

    #[derive(Template)]
    #[template(path = "home.html")]
    struct HomePage;

    HomePage
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
        // .configure(events::config)
    })
    .bind((host, port))?
    .run()
    .await
}
