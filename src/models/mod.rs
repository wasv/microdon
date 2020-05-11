/// Database models for activities.
mod activities;
pub use self::activities::Activity;
/// Database models for encountered actors and follow connections.
mod actors;
pub use self::actors::{Actor, Following};
/// Database models for objects.
mod objects;
pub use self::objects::Object;

fn fetch(id: String) -> Result<serde_json::Value, String> {
    reqwest::blocking::Client::new()
        .get(&id)
        .header(reqwest::header::ACCEPT, "application/activity+json")
        .send()
        .and_then(|r| r.json::<serde_json::Value>())
        .or(Err("Could not get activity.".to_string()))
}
