extern crate actix_rt;
extern crate actix_web;
extern crate futures;
extern crate serde_json;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();

    HttpServer::new(move || {
        let actor_id = env::var("SELF").unwrap_or_else(|_| "http://localhost:8000/".to_string());
        App::new()
            .wrap(Logger::default())
            .data(api::State::new(actor_id))
            .service(
                web::resource("/inbox")
                    .route(web::post().to(api::inbox::post))
                    .route(web::get().to(api::inbox::get)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
