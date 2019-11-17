use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::actors;

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "actors"]
pub struct Actor {
    pub id: String,
    pub username: String,
    pub profile: String,
}

pub fn create_actor(actor: Actor, connection: &Conn) -> Option<Actor> {
    diesel::insert_into(actors::table)
        .values(&actor)
        .get_result(connection)
        .ok()
}

pub fn read_actor(id: String, connection: &Conn) -> Option<Actor> {
    actors::table.find(id).first(connection).ok()
}

pub fn list_actors(connection: &Conn) -> Vec<Actor> {
    actors::table
        .order(actors::id.asc())
        .load::<Actor>(connection)
        .unwrap_or(Vec::new())
}

pub fn update_actor(actor: Actor, connection: &Conn) -> bool {
    diesel::update(actors::table.find(actor.id.clone()))
        .set(actor)
        .execute(connection)
        .is_ok()
}

pub fn delete_actor(id: String, connection: &Conn) -> bool {
    diesel::delete(actors::table.find(id))
        .execute(connection)
        .is_ok()
}
