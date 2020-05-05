use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::inbox;

/// A model for a new activity in the inbox.
#[derive(Debug, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "inbox"]
pub struct InboxActivity {
    /// The id of the activity as a URL to the ActivityPub activity document.
    pub id: String,
    /// Foreign key to the author of the activity, in the form of a URL to the ActivityPub Actor document.
    pub actor: String,
    /// The contents of the activity.
    pub payload: Option<serde_json::Value>,
}

/// Inserts an inbox activity into the database.
pub fn insert_inbox_activity(
    inbox_activity: InboxActivity,
    connection: &Conn,
) -> QueryResult<InboxActivity> {
    diesel::insert_into(inbox::table)
        .values(&inbox_activity)
        .on_conflict_do_nothing()
        .execute(connection)?;
    read_inbox_activity(inbox_activity.id, connection)
}

/// Reads an activity from the inbox based on the activity's id.
pub fn read_inbox_activity(id: String, connection: &Conn) -> QueryResult<InboxActivity> {
    inbox::table.find(id).first(connection)
}

/// Lists all activities in the inbox.
pub fn list_inbox(connection: &Conn) -> Vec<InboxActivity> {
    inbox::table
        .order(inbox::id.asc())
        .load::<InboxActivity>(connection)
        .unwrap_or_default()
}

/// Updates an activity in the inbox.
pub fn update_inbox_activity(inbox_activity: InboxActivity, connection: &Conn) -> bool {
    diesel::update(inbox::table.find(inbox_activity.id.clone()))
        .set(inbox_activity)
        .execute(connection)
        .is_ok()
}

/// Deletes an activity from the inbox.
pub fn delete_inbox_activity(id: String, connection: &Conn) -> bool {
    diesel::delete(inbox::table.find(id))
        .execute(connection)
        .is_ok()
}
