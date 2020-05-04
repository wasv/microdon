use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::outbox;

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "outbox"]
pub struct OutboxActivity {
    pub id: String,
    pub payload: Option<serde_json::Value>,
}

pub fn create_outbox_activity(
    outbox_activity: OutboxActivity,
    connection: &Conn,
) -> QueryResult<OutboxActivity> {
    diesel::insert_into(outbox::table)
        .values(&outbox_activity)
        .execute(connection)?;
    read_outbox_activity(outbox_activity.id, connection)
}

pub fn read_outbox_activity(id: String, connection: &Conn) -> QueryResult<OutboxActivity> {
    outbox::table.find(id).first(connection)
}

pub fn list_outbox(connection: &Conn) -> Vec<OutboxActivity> {
    outbox::table
        .order(outbox::id.asc())
        .load::<OutboxActivity>(connection)
        .unwrap_or(Vec::new())
}

pub fn update_outbox_activity(outbox_activity: OutboxActivity, connection: &Conn) -> bool {
    diesel::update(outbox::table.find(outbox_activity.id.clone()))
        .set(outbox_activity)
        .execute(connection)
        .is_ok()
}

pub fn delete_outbox_activity(id: String, connection: &Conn) -> bool {
    diesel::delete(outbox::table.find(id))
        .execute(connection)
        .is_ok()
}
