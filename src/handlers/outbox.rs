use std::result::Result;

use serde_json::Value;

use crate::connection::DbConn;
use crate::models::Activity;

/// Handles a new create activity.
pub fn create(db: DbConn, contents: Value) -> Result<Activity, String> {
    let activity = Activity::get(contents, &db)?.insert(&db)?;
    Ok(activity)
}
