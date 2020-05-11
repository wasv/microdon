use std::time::SystemTime;

use serde_json::Value;

use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl};

use super::fetch;
use crate::connection::Conn;
use crate::schema::{actors, followings};

/// A model for storing an actor in a database.
#[derive(Debug, Identifiable, Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "actors"]
pub struct Actor {
    /// The id of the actor as the URL to the ActivityPub Actor document.
    pub id: String,
}

/// A model for storing an actor who follows the user in a database.
#[derive(Debug, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "followings"]
pub struct Following {
    /// Foreign key to the person being followed, in the form of a URL to original Actor object.
    pub target: String,
    /// Timestamp of when this Actor followed the user.
    pub since: SystemTime,
    /// Foreign key to the actor following the target, in the form of a URL to original Actor object.
    pub follower: String,
}

impl Actor {
    pub fn get(contents: Value, db: &Conn) -> Result<Self, String> {
        let contents = match contents {
            Value::String(id) => match Self::read(id.clone(), db) {
                Ok(object) => return Ok(object),
                _ => fetch(id)?
                    .as_object()
                    .ok_or("Invalid activity reference.".to_string())?
                    .to_owned(),
            },
            Value::Object(contents) => contents,
            _ => return Err("Invalid activity reference.".to_string()),
        };

        let id = contents["id"].as_str().ok_or("No object id")?.to_string();
        Ok(Actor { id })
    }

    pub fn read(id: String, db: &Conn) -> Result<Self, String> {
        Self::table()
            .find(id)
            .first(db)
            .or_else(|e| Err(format!("Could not read actor. {}", e)))
    }
    pub fn list(db: &Conn) -> Vec<Self> {
        Self::table().load::<Self>(db).unwrap_or_default()
    }

    pub fn insert(&self, db: &Conn) -> Result<Self, String> {
        diesel::insert_into(Self::table())
            .values(self)
            .on_conflict_do_nothing()
            .execute(db)
            .or_else(|e| Err(format!("Could not insert actor. {}", e)))?;
        Self::read(self.id.clone(), db)
            .or_else(|e| Err(format!("Could not read inserted actor. {}", e)))
    }
    pub fn remove(&self, db: &Conn) -> bool {
        diesel::delete(Self::table().find(self.id.clone()))
            .execute(db)
            .is_ok()
    }
}
