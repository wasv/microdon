use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::inbox;

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "inbox"]
pub struct InboxActivity {
    pub id: String,
    pub payload: Option<serde_json::Value>,
}

pub fn create_inbox_activity(
    inbox_activity: InboxActivity,
    connection: &Conn,
) -> Option<InboxActivity> {
    diesel::insert_into(inbox::table)
        .values(&inbox_activity)
        .get_result(connection)
        .ok()
}

pub fn read_inbox_activity(id: String, connection: &Conn) -> Option<InboxActivity> {
    inbox::table.find(id).first(connection).ok()
}

pub fn list_inbox(connection: &Conn) -> Vec<InboxActivity> {
    inbox::table
        .order(inbox::id.asc())
        .load::<InboxActivity>(connection)
        .unwrap_or(Vec::new())
}

pub fn update_inbox_activity(inbox_activity: InboxActivity, connection: &Conn) -> bool {
    diesel::update(inbox::table.find(inbox_activity.id.clone()))
        .set(inbox_activity)
        .execute(connection)
        .is_ok()
}

pub fn delete_inbox_activity(id: String, connection: &Conn) -> bool {
    diesel::delete(inbox::table.find(id))
        .execute(connection)
        .is_ok()
}
