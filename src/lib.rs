//! Support library for microdon.
//!
//! Handlers and database models for working with ActivityPub documents.
#![warn(missing_docs)]

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

/// Contains the supporting methods and structs for the database connection.
pub mod connection;
/// Contains the handlers for the various types of activities.
pub mod handlers;
/// Contains the models and the database objects.
pub mod models;
/// Contains the database schemas.
#[allow(missing_docs)]
pub mod schema;
