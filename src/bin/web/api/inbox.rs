use microdon::connection::DbConn;
use microdon::handlers::inbox;
use microdon::models::Activity;

use rocket_contrib::json::Json;

#[post("/", format = "json", data = "<data>")]
pub fn post(data: String, connection: DbConn) -> Result<(), String> {
    serde_json::from_str(&data)
        .or(Err("JSON Error".to_string()))
        .and_then(|data| {
            inbox::create(connection, data)
                .and(Ok(()))
        })
}

#[get("/")]
pub fn get(connection: DbConn) -> Json<Vec<Activity>> {
    Json(Activity::list(&connection))

}
