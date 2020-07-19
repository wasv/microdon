extern crate actix_rt;
extern crate actix_web;
extern crate futures;
extern crate serde_json;

use actix_web::{web, App, HttpServer};

mod api;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().data(api::State::new()).service(
            web::resource("/inbox")
                .route(web::post().to(api::inbox::post))
                .route(web::get().to(api::inbox::get)),
        )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
