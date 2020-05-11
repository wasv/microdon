#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

use microdon::connection;

mod api;

fn main() {
    rocket::ignite()
        .mount("/inbox", routes![api::inbox::post, api::inbox::get,])
        .register(catchers![not_found])
        .manage(connection::init_pool())
        .launch();
}

#[catch(404)]
fn not_found(_req: &Request) -> Json<JsonValue> {
    Json(json!({ "error": "Not found" }))
}
