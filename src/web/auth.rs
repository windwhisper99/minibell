use actix_web::{
    web::{get, scope, Data, Query, Redirect, ServiceConfig},
    Responder,
};
use serde::Deserialize;

use crate::utils::{db::Database, discord::DiscordClient};

#[derive(Deserialize)]
struct RedirectQuery {
    code: String,
}

async fn redirect(
    query: Query<RedirectQuery>,
    discord_client: Data<DiscordClient>,
    client: Data<reqwest::Client>,
    db: Data<Database>,
) -> impl Responder {
    let member = match discord_client.auth(&query.code, &client, &db).await {
        Ok(member) => member,
        Err(err) => {
            println!("{:?}", err);
            return Redirect::to("/").temporary();
        }
    };
    println!("{:?}", member);
    println!("{}", discord_client.avatar_url(&member));

    Redirect::to("/").temporary()
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("auth").route("/redirect", get().to(redirect)));
}
