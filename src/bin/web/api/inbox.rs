use actix_web::{error, web, Responder};

use super::State;

use microdon::connection::DbConn;
use microdon::handlers::inbox;
use microdon::models::Activity;

pub async fn post(body_stream: web::Payload, state: web::Data<State>) -> impl Responder {
    let activity = super::parse_body(body_stream).await?;
    let db = state.get_db();

    inbox::create(db, activity)
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Create failed {}", e)))
        .map(web::Json)
}

pub async fn get(state: web::Data<State>) -> impl Responder {
    let db = DbConn(state.db.get().unwrap());

    web::Json(Activity::list(&db))
}
