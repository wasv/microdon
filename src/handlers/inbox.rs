use std::result::Result;

use crate::connection::DbConn;
use crate::models::actors::*;
use crate::models::inbox::*;

pub fn create(connection: DbConn, payload: serde_json::Value) -> Result<InboxActivity, String> {
    let activity = add_to_inbox(connection, payload.clone())?;
    forward_from_inbox(payload)?;
    Ok(activity)
}
fn forward_from_inbox(_payload: serde_json::Value) -> Result<(), String> {
    Ok(())
}
fn add_to_inbox(connection: DbConn, payload: serde_json::Value) -> Result<InboxActivity, String> {
    let activity_id = payload["id"].as_str().ok_or("No activity id")?;
    let actor_id = payload["actor"].as_str().ok_or("No actor id")?;

    let actor = get_actor(actor_id, &connection).or_else(|e| Err(format!("Couldn't get actor {}", e)))?;

    trace!("Actor:\n {:#?}", actor);

    let activity = create_inbox_activity(
        InboxActivity {
            id: activity_id.to_string(),
            actor: actor.id.to_string(),
            payload: Some(payload),
        },
        &connection,
    )
    .or(Err("DB Insert Failed"))?;
    Ok(activity)
}
