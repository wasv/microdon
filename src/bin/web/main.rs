#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

use satchel::connection;

mod api;

fn main() {
    rocket::ignite()
        .mount(
            "/actors",
            routes![
                api::actors::create,
                api::actors::list_all,
                api::actors::read,
                api::actors::update,
                api::actors::delete
            ],
        )
        .mount(
            "/inbox",
            routes![
                api::inbox::create,
                api::inbox::list_all,
                api::inbox::read,
                api::inbox::update,
                api::inbox::delete
            ],
        )
        .mount(
            "/outbox",
            routes![
                api::outbox::create,
                api::outbox::list_all,
                api::outbox::read,
                api::outbox::update,
                api::outbox::delete
            ],
        )
        .register(catchers![not_found])
        .manage(connection::init_pool())
        .launch();
}

#[catch(404)]
fn not_found(_req: &Request) -> Json<JsonValue> {
    Json(json!({ "error": "Not found" }))
}
