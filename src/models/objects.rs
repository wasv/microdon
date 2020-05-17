use std::time::SystemTime;

use serde_json::Value;

use diesel::associations::HasTable;
use diesel::{QueryDsl, QueryResult, RunQueryDsl};

use super::actors::Actor;
use super::fetch;

use crate::connection::Conn;
use crate::schema::objects;

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct Object {
    /// The id of the activity as a URL to the original activity.
    pub id: String,
    /// The type of the activity.
    pub objtype: String,
    /// Foreign key to the author of the activity, in the form of a URL to original Actor object.
    pub author: String,
    /// Time activity was published if available.
    pub published: Option<SystemTime>,
    /// The contents of the activity.
    pub contents: Option<serde_json::Value>,
}

impl Object {
    pub fn get(contents: Value, db: &Conn) -> Result<Self, String> {
        let contents = match contents {
            Value::String(id) => match Self::read(id.clone(), db) {
                Ok(object) => return Ok(object),
                _ => fetch(id)?
                    .as_object()
                    .ok_or_else(|| "Invalid activity reference.".to_string())?
                    .to_owned(),
            },
            Value::Object(contents) => contents,
            _ => return Err("Invalid activity reference.".to_string()),
        };

        let id = contents["id"].as_str().ok_or("No object id")?.to_string();
        let objtype = contents["type"]
            .as_str()
            .ok_or("No type field found in object")?
            .to_string();
        let actor = Actor::get(contents["attributedTo"].clone(), &db)?;

        Ok(Object {
            id,
            objtype,
            author: actor.id,
            published: None,
            contents: Some(Value::Object(contents)),
        })
    }

    pub fn read(id: String, db: &Conn) -> QueryResult<Self> {
        Self::table().find(id).first(db)
    }
    pub fn list(db: &Conn) -> Vec<Self> {
        Self::table().load::<Self>(db).unwrap_or_default()
    }

    pub fn insert(&self, db: &Conn) -> Result<Self, String> {
        Actor::get(Value::String(self.author.clone()), &db)?
            .insert(&db)
            .or_else(|e| Err(format!("Could not insert author. {}", e)))?;

        diesel::insert_into(Self::table())
            .values(self)
            .on_conflict_do_nothing()
            .execute(db)
            .or_else(|e| Err(format!("Could not insert object. {}", e)))?;
        Self::read(self.id.clone(), db)
            .or_else(|e| Err(format!("Could not read inserted object. {}", e)))
    }
    pub fn update(&self, db: &Conn) -> QueryResult<Self> {
        diesel::update(Self::table().find(self.id.clone()))
            .set(self)
            .execute(db)?;
        Self::read(self.id.clone(), db)
    }
    pub fn remove(&self, db: &Conn) -> bool {
        diesel::delete(Self::table().find(self.id.clone()))
            .execute(db)
            .is_ok()
    }
}
