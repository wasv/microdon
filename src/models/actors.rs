use std::time::SystemTime;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::{actors, followers, following};

#[derive(Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "actors"]
pub struct Actor {
    pub id: String,
    pub username: String,
    pub profile: String,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "followers"]
pub struct Follower {
    pub actor: String,
    pub since: SystemTime,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "following"]
pub struct FollowedActor {
    pub actor: String,
    pub since: SystemTime,
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

pub fn add_follower(actor: Actor, connection: &Conn) -> Option<Follower> {
    diesel::insert_into(followers::table)
        .values(Follower {
            actor: actor.id,
            since: SystemTime::now(),
        })
        .get_result(connection)
        .ok()
}

pub fn follow_actor(actor: Actor, connection: &Conn) -> Option<FollowedActor> {
    diesel::insert_into(following::table)
        .values(FollowedActor {
            actor: actor.id,
            since: SystemTime::now(),
        })
        .get_result(connection)
        .ok()
}

pub fn list_followers(connection: &Conn) -> Vec<(Follower, Actor)> {
    followers::table
        .order(followers::since.asc())
        .inner_join(actors::table)
        .load::<(Follower, Actor)>(connection)
        .unwrap_or(Vec::new())
}

pub fn list_following(connection: &Conn) -> Vec<(FollowedActor, Actor)> {
    following::table
        .order(following::since.asc())
        .inner_join(actors::table)
        .load::<(FollowedActor, Actor)>(connection)
        .unwrap_or(Vec::new())
}
