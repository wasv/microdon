use rocket_contrib::json::{Json, JsonValue};

use microdon::connection;
use microdon::models::outbox::*;

#[post("/", data = "<activity>")]
pub fn create(
    activity: Json<OutboxActivity>,
    connection: connection::DbConn,
) -> Option<Json<OutboxActivity>> {
    let insert = OutboxActivity {
        ..activity.into_inner()
    };
    create_outbox_activity(insert, &connection)
        .and_then(|activity| Ok(Json(activity)))
        .ok()
}

#[get("/")]
pub fn list_all(connection: connection::DbConn) -> Json<Vec<OutboxActivity>> {
    Json(list_outbox(&connection))
}

#[get("/<id>")]
pub fn read(id: String, connection: connection::DbConn) -> Option<Json<OutboxActivity>> {
    read_outbox_activity(id, &connection)
        .and_then(|activity| Ok(Json(activity)))
        .ok()
}

#[put("/", data = "<activity>")]
pub fn update(activity: Json<OutboxActivity>, connection: connection::DbConn) -> Json<JsonValue> {
    let update = OutboxActivity {
        ..activity.into_inner()
    };
    Json(json!({
        "success": update_outbox_activity(update, &connection)
    }))
}

#[delete("/<id>")]
pub fn delete(id: String, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!({
        "success": delete_outbox_activity(id, &connection)
    }))
}
