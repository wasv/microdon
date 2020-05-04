#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate diesel_migrations;
extern crate r2d2;
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate log;

pub mod connection;
pub mod handlers;
pub mod models;
pub mod schema;
