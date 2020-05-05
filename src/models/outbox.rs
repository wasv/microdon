use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::outbox;

/// A model for a new activity in the outbox.
///
/// Does not contain an author ID, since it is published by the user.
#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "outbox"]
pub struct OutboxActivity {
    /// The id of the activity as a URL to the ActivityPub activity document.
    pub id: String,
    /// The contents of the activity.
    pub payload: Option<serde_json::Value>,
}

/// Inserts an outbox activity into the database.
pub fn insert_outbox_activity(
    outbox_activity: OutboxActivity,
    connection: &Conn,
) -> QueryResult<OutboxActivity> {
    diesel::insert_into(outbox::table)
        .values(&outbox_activity)
        .execute(connection)?;
    read_outbox_activity(outbox_activity.id, connection)
}

/// Reads an activity from the outbox based on the activity's id.
pub fn read_outbox_activity(id: String, connection: &Conn) -> QueryResult<OutboxActivity> {
    outbox::table.find(id).first(connection)
}

/// Lists all activities in the outbox.
pub fn list_outbox(connection: &Conn) -> Vec<OutboxActivity> {
    outbox::table
        .order(outbox::id.asc())
        .load::<OutboxActivity>(connection)
        .unwrap_or(Vec::new())
}

/// Updates an activity in the outbox.
pub fn update_outbox_activity(outbox_activity: OutboxActivity, connection: &Conn) -> bool {
    diesel::update(outbox::table.find(outbox_activity.id.clone()))
        .set(outbox_activity)
        .execute(connection)
        .is_ok()
}

/// Deletes an activity from the outbox.
pub fn delete_outbox_activity(id: String, connection: &Conn) -> bool {
    diesel::delete(outbox::table.find(id))
        .execute(connection)
        .is_ok()
}
