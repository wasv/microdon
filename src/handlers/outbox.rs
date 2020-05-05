use std::result::Result;

use crate::connection::DbConn;
use crate::models::outbox::*;

/// Handles a new create activity.
pub fn create(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    add_to_outbox(connection, payload)
}

/// Add a new activity to the outbox.
fn add_to_outbox(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    let activity_id = payload["id"].as_str().ok_or("No activity id")?;

    insert_outbox_activity(
        OutboxActivity {
            id: activity_id.to_string(),
            payload: Some(payload),
        },
        &connection,
    )
    .or(Err("DB Insert Failed"))?;
    Ok(())
}
