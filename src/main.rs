use actix_web::{App, HttpServer};
use crate::config::db::create_session;
use crate::routes::users::init;

mod routes;
mod config;
mod handlers;
mod responses;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let session = create_session().await;

    HttpServer::new(move || {
        App::new()
            .app_data(session.clone())
            .configure(init)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
