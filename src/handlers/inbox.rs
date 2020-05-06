use std::result::Result;

use crate::connection::DbConn;
use crate::models::actors::*;
use crate::models::inbox::*;

/// Handles a new create activity.
pub fn create(connection: DbConn, payload: serde_json::Value) -> Result<InboxActivity, String> {
    let activity = add_to_inbox(connection, payload.clone())?;
    forward_from_inbox(payload)?;
    Ok(activity)
}

/// Forwards an activity from the inbox.
fn forward_from_inbox(_payload: serde_json::Value) -> Result<(), String> {
    Ok(()) // Stubbed for now.
}

/// Add a new activity to the inbox.
///
/// Also creates an Actor if the author of the activity does not yet exist.
fn add_to_inbox(connection: DbConn, payload: serde_json::Value) -> Result<InboxActivity, String> {
    let activity_id = payload["id"].as_str().ok_or("No activity id")?;
    let actor_id = payload["actor"].as_str().ok_or("No actor id")?;

    let actor = read_actor(actor_id.to_string(), &connection).or_else(|_| {
        get_actor(actor_id.to_string(), &connection).or_else(|e| Err(format!("Couldn't get actor {}", e)))
    })?;

    trace!("Actor:\n {:#?}", actor);

    let activity = insert_inbox_activity(
        InboxActivity {
            id: activity_id.to_string(),
            actor: actor.id,
            payload: Some(payload),
        },
        &connection,
    )
    .or(Err("DB Insert Failed"))?;
    Ok(activity)
}
