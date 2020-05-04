use std::result::Result;

use crate::connection::DbConn;
use crate::models::outbox::*;

pub fn create(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    add_to_outbox(connection, payload)
}
fn add_to_outbox(connection: DbConn, payload: serde_json::Value) -> Result<(), String> {
    let activity_id = payload["id"].as_str().ok_or("No activity id")?;

    create_outbox_activity(
        OutboxActivity {
            id: activity_id.to_string(),
            payload: Some(payload),
        },
        &connection,
    )
    .or(Err("DB Insert Failed"))?;
    Ok(())
}
