use actix_web::{error, web, Responder};

use super::State;

use microdon::handlers::inbox;

pub async fn post(body_stream: web::Payload, state: web::Data<State>) -> impl Responder {
    let activity = super::parse_body(body_stream).await?;
    let db = state.get_db();

    inbox::create(db, activity)
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Create failed {}", e)))
        .map(web::Json)
}

pub async fn get(state: web::Data<State>) -> impl Responder {
    web::Json(inbox::get_all(state.get_db(), state.get_actor_id()))
}
