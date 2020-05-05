use rocket_contrib::json::{Json, JsonValue};

use microdon::connection;
use microdon::models::actors::*;

#[post("/", data = "<actor>")]
pub fn create(actor: Json<Actor>, connection: connection::DbConn) -> Option<Json<Actor>> {
    let insert = Actor {
        ..actor.into_inner()
    };
    insert_actor(insert, &connection).map(Json).ok()
}

#[get("/")]
pub fn list_all(connection: connection::DbConn) -> Json<Vec<Actor>> {
    Json(list_actors(&connection))
}

#[get("/<id>")]
pub fn read(id: String, connection: connection::DbConn) -> Option<Json<Actor>> {
    read_actor(id, &connection).map(Json).ok()
}

#[put("/", data = "<actor>")]
pub fn update(actor: Json<Actor>, connection: connection::DbConn) -> Json<JsonValue> {
    let update = Actor {
        ..actor.into_inner()
    };
    Json(json!({ "success": update_actor(update, &connection) }))
}

#[delete("/<id>")]
pub fn delete(id: String, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!({ "success": delete_actor(id, &connection) }))
}
