use actix_web::{error, web, Responder};
use futures::StreamExt;

use super::State;

use microdon::connection::DbConn;
use microdon::handlers::inbox;
use microdon::models::Activity;

pub async fn post(mut body_stream: web::Payload, state: web::Data<State>) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(item) = body_stream.next().await {
        body.extend_from_slice(&item?);
    }
    let body = body.freeze();

    let db = DbConn(state.db.get().unwrap());

    let json_act = serde_json::from_slice(&body[..])
        .map_err(|e| error::ErrorBadRequest(format!("JSON Error {}", e)))?;

    inbox::create(db, json_act)
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("Create failed {}", e)))
        .and_then(|data| Ok(web::Json(data)))
}

pub async fn get(state: web::Data<State>) -> impl Responder {
    let db = DbConn(state.db.get().unwrap());

    web::Json(Activity::list(&db))
}
