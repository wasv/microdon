use microdon::connection::DbConn;
use microdon::handlers::inbox;
use microdon::models::Activity;

use rocket::Data;
use rocket_contrib::json::Json;

#[post("/", data = "<data>")]
pub fn post(data: Data, connection: DbConn) -> Result<Json<Activity>, String> {
    serde_json::from_reader(data.open())
        .or_else(|e| Err(format!("JSON Error {}", e)))
        .and_then(|data| inbox::create(connection, data).and_then(|a| Ok(Json(a))))
}

#[get("/")]
pub fn get(connection: DbConn) -> Json<Vec<Activity>> {
    Json(Activity::list(&connection))
}
