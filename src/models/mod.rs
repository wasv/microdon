/// Database models for activities.
mod activities;
pub use self::activities::Activity;
/// Database models for encountered actors and follow connections.
mod actors;
pub use self::actors::{Actor, Following};
/// Database models for objects.
mod objects;
pub use self::objects::Object;

/// Helper function for retrieving ActivityPub objects.
async fn fetch(id: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();

    client
        .get(&id)
        .header(reqwest::header::ACCEPT, "application/activity+json")
        .send()
        .await
        .map_err(|e| format!("Could not get object: {}", e))?
        .json::<serde_json::Value>()
        .await
        .map_err(|e| format!("Could parse json: {}", e))
}
