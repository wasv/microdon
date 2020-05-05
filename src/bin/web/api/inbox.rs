use rocket_contrib::json::{Json, JsonValue};

use microdon::connection;
use microdon::models::inbox::*;

#[post("/", data = "<activity>")]
pub fn create(
    activity: Json<InboxActivity>,
    connection: connection::DbConn,
) -> Option<Json<InboxActivity>> {
    let insert = InboxActivity {
        ..activity.into_inner()
    };
    insert_inbox_activity(insert, &connection).map(Json).ok()
}

#[get("/")]
pub fn list_all(connection: connection::DbConn) -> Json<Vec<InboxActivity>> {
    Json(list_inbox(&connection))
}

#[get("/<id>")]
pub fn read(id: String, connection: connection::DbConn) -> Option<Json<InboxActivity>> {
    read_inbox_activity(id, &connection).map(Json).ok()
}

#[put("/", data = "<activity>")]
pub fn update(activity: Json<InboxActivity>, connection: connection::DbConn) -> Json<JsonValue> {
    let update = InboxActivity {
        ..activity.into_inner()
    };
    Json(json!({
        "success": update_inbox_activity(update, &connection)
    }))
}

#[delete("/<id>")]
pub fn delete(id: String, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!({ "success": delete_inbox_activity(id, &connection) }))
}
