use actix_web::{
    middleware,
    web::{get, resource, Data},
    App, HttpServer, Responder,
};
use askama::Template;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Clone)]
struct Database {
    surrel: Surreal<Client>,
}

async fn home_page() -> impl Responder {
    #[derive(Template)]
    #[template(path = "home.html")]
    struct HomePage;

    HomePage
}

pub async fn run(host: String, port: u16) -> std::io::Result<()> {
    let surrel = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    surrel
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();

    let db = Database { surrel };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .wrap(middleware::NormalizePath::default())
            .wrap(middleware::Compress::default())
            .service(actix_files::Files::new("/assets", "assets").use_last_modified(true))
            .service(resource("/").route(get().to(home_page)))
    })
    .bind((host, port))?
    .run()
    .await
}
