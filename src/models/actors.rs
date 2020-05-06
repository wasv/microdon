use std::time::SystemTime;

use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::{actors, followers, following};

/// A model for storing an actor in a database.
#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "actors"]
pub struct Actor {
    /// The id of the actor as the URL to the ActivityPub Actor document.
    pub id: String,
    /// The username of the actor. preferredUsername from JSON actor.
    pub username: String,
    /// The URL to the users human readable profile.
    pub profile: String,
}

/// A model for storing an actor who follows the user in a database.
#[derive(Debug, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "followers"]
pub struct Follower {
    /// Foreign key to the associated actor, in the form of a URL to the ActivityPub Actor document.
    pub actor: String,
    /// Timestamp of when this Actor followed the user.
    pub since: SystemTime,
}

/// A model for representing an actor the user follows.
#[derive(Debug, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "following"]
pub struct FollowedActor {
    /// Foreign key to the associated actor, in the form of a URL to the ActivityPub Actor document.
    pub actor: String,
    /// Timestamp of when the user followed this Actor.
    pub since: SystemTime,
}

/// Retrieves, parses, and inserts an actor given an ID.
///
/// Meant to be used alongside [`read_actor`].
///
/// ```
/// let actor_id = "https://mastodon.social/users/Gargon";
///
/// let actor = read_actor(id, connection).or_else(|| get_actor(id, connection));
/// ```
pub fn get_actor(id: String, connection: &Conn) -> Result<Actor, String> {
    let actor: serde_json::Value = reqwest::blocking::Client::new()
        .get(&id)
        .header(reqwest::header::ACCEPT, "application/activity+json")
        .send()
        .and_then(|r| r.json())
        .or(Err("Could not get actor."))?;
    let name = actor["preferredUsername"]
        .as_str()
        .ok_or("No actor username")?;
    let url = actor["url"].as_str().ok_or("No actor url")?;
    let actor = insert_actor(
        Actor {
            id,
            username: name.to_string(),
            profile: url.to_string(),
        },
        &connection,
    )
    .or(Err("Cannot create actor."))?;
    Ok(actor)
}

/// Inserts an actor into the database.
///
/// Not recommended for direct use. Use [`get_actor`] instead.
pub fn insert_actor(actor: Actor, connection: &Conn) -> QueryResult<Actor> {
    diesel::insert_into(actors::table)
        .values(&actor)
        .on_conflict_do_nothing()
        .execute(connection)?;
    read_actor(actor.id, connection)
}

/// Reads an actor from the database table based on the Actor's id.
///
/// Meant to be used alongside [`get_actor`].
///
/// ```
/// let actor_id = "https://mastodon.social/users/Gargon";
///
/// let actor = read_actor(id, connection).or_else(|_| get_actor(id, connection));
/// ```
pub fn read_actor(id: String, connection: &Conn) -> QueryResult<Actor> {
    actors::table.find(id).first(connection)
}

/// Lists all actors in the database.
pub fn list_actors(connection: &Conn) -> Vec<Actor> {
    actors::table
        .order(actors::id.asc())
        .load::<Actor>(connection)
        .unwrap_or_default()
}

/// Updates an actor in the database.
pub fn update_actor(actor: Actor, connection: &Conn) -> bool {
    diesel::update(actors::table.find(actor.id.clone()))
        .set(actor)
        .execute(connection)
        .is_ok()
}

/// Deletes an actor from the database.
pub fn delete_actor(id: String, connection: &Conn) -> bool {
    diesel::delete(actors::table.find(id))
        .execute(connection)
        .is_ok()
}

/// Add an actor as a new follower.
pub fn add_follower(actor: Actor, connection: &Conn) -> Option<Follower> {
    diesel::insert_into(followers::table)
        .values(Follower {
            actor: actor.id,
            since: SystemTime::now(),
        })
        .get_result(connection)
        .ok()
}

/// Removes a follower when the actor unfollows the user.
pub fn delete_follower(actor: Actor, connection: &Conn) -> bool {
    diesel::delete(followers::table.find(actor.id))
        .execute(connection)
        .is_ok()
}

/// Lists all of the users followers from the database.
pub fn list_followers(connection: &Conn) -> Vec<(Follower, Actor)> {
    followers::table
        .order(followers::since.asc())
        .inner_join(actors::table)
        .load::<(Follower, Actor)>(connection)
        .unwrap_or_default()
}

/// Adds an actor that the user follows.
pub fn follow_actor(actor: Actor, connection: &Conn) -> Option<FollowedActor> {
    diesel::insert_into(following::table)
        .values(FollowedActor {
            actor: actor.id,
            since: SystemTime::now(),
        })
        .get_result(connection)
        .ok()
}

/// Removes an actor that the user no longer follows.
pub fn unfollow_actor(actor: Actor, connection: &Conn) -> bool {
    diesel::delete(following::table.find(actor.id))
        .execute(connection)
        .is_ok()
}

/// Lists all of the actors that the user follows from the database.
pub fn list_following(connection: &Conn) -> Vec<(FollowedActor, Actor)> {
    following::table
        .order(following::since.asc())
        .inner_join(actors::table)
        .load::<(FollowedActor, Actor)>(connection)
        .unwrap_or_default()
}
