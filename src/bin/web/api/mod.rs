pub mod inbox;

use actix_web::{error, web};
use futures::StreamExt;
use microdon::connection;
use microdon::connection::DbConn;

pub struct State {
    db: connection::Pool,
    actor_id: String,
}

impl State {
    pub fn new(actor_id: String) -> State {
        State {
            db: connection::init_pool(),
            actor_id: actor_id,
        }
    }

    pub fn get_db(&self) -> DbConn {
        DbConn(self.db.get().unwrap())
    }

    pub fn get_actor_id(&self) -> String {
        self.actor_id.clone()
    }
}

pub async fn parse_body(mut stream: web::Payload) -> Result<serde_json::Value, actix_web::Error> {
    let mut body = web::BytesMut::new();
    while let Some(item) = stream.next().await {
        body.extend_from_slice(&item?);
    }
    let body = body.freeze();

    serde_json::from_slice(&body[..])
        .map_err(|e| error::ErrorBadRequest(format!("JSON Error {}", e)))
}
