use reqwest::header;
use std::result::Result;

use crate::connection::DbConn;
use crate::models::actors::*;
use crate::models::inbox::*;

pub fn create(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    let activity_id = payload["id"].as_str().ok_or("No activity id")?;
    let actor_id = payload["actor"].as_str().ok_or("No actor id")?;

    if read_actor(actor_id.to_string(), &connection).is_none() {
        let actor: serde_json::Value = reqwest::blocking::Client::new()
            .get(actor_id)
            .header(header::ACCEPT, "application/activity+json")
            .send()
            .and_then(|r| r.json())
            .or(Err("Could not get actor."))?;
        let actor_name = actor["name"].as_str().ok_or("No actor name")?;
        let actor_url = actor["url"].as_str().ok_or("No actor url")?;
        create_actor(
            Actor {
                id: actor_id.to_string(),
                username: actor_name.to_string(),
                profile: actor_url.to_string(),
            },
            &connection,
        );
    }

    create_inbox_activity(
        InboxActivity {
            id: activity_id.to_string(),
            actor: actor_id.to_string(),
            payload: Some(payload),
        },
        &connection,
    )
    .or(Err("DB Insert Failed"))?;
    Ok(())
}
pub fn announce(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    Err("NI".to_string())
}
pub fn delete(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    Err("NI".to_string())
}
pub fn follow(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    Err("NI".to_string())
}
pub fn accept(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    Err("NI".to_string())
}
