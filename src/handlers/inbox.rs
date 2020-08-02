use std::result::Result;

use serde_json::Value;

use crate::connection::DbConn;
use crate::models::Activity;

use super::OrderedCollection;

/// Handles a new create activity.
pub async fn create(db: DbConn, contents: Value) -> Result<Activity, String> {
    let activity = Activity::get(contents.clone(), &db)
        .await?
        .insert(&db)
        .await?;
    forward_from_inbox(contents)?;
    Ok(activity)
}

/// Forwards an activity from the inbox.
fn forward_from_inbox(_payload: Value) -> Result<(), String> {
    Ok(()) // Stubbed for now.
}

/// Lists all known activities
pub fn get_all(db: DbConn, actor_id: String) -> OrderedCollection<serde_json::Value> {
    let mut activities = Activity::list(&db);

    activities.sort_by(|a, b| b.published.cmp(&a.published));
    let items: Vec<_> = activities.iter().map(|a| a.contents.clone()).collect();

    OrderedCollection {
        id: format!("{}/inbox", actor_id),
        items: items,
    }
}
