use std::time::SystemTime;

use serde_json::Value;

use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl};

use super::actors::Actor;
use super::fetch;
use super::objects::Object;

use crate::connection::Conn;
use crate::schema::activities;

/// A model for a stored activity.
#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "activities"]
pub struct Activity {
    /// The id of the activity as a URL to the original activity.
    pub id: String,
    /// The type of the activity.
    pub acttype: String,
    /// Foreign key to the author of the activity, in the form of a URL to original Actor object.
    pub author: String,
    /// Time activity was published if available.
    pub published: Option<SystemTime>,
    /// The id of the associated object, as a URL to the original object.
    pub object: String,
    /// The contents of the activity.
    pub contents: Option<Value>,
}

impl Activity {
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

        let activity_id = contents["id"].as_str().ok_or("No activity id")?;
        let actor = Actor::get(contents["actor"].clone(), &db)?;
        let object = Object::get(contents["object"].clone(), &db)?;

        let acttype = contents["type"]
            .as_str()
            .ok_or("No type field found in activity")?
            .to_string();

        Ok(Activity {
            id: activity_id.to_string(),
            acttype,
            author: actor.id,
            object: object.id,
            contents: Some(Value::Object(contents.to_owned())),
            published: None,
        })
    }

    pub fn read(id: String, db: &Conn) -> Result<Self, String> {
        Self::table()
            .find(id)
            .first(db)
            .or_else(|e| Err(format!("Could not read activity. {}", e)))
    }
    pub fn list(db: &Conn) -> Vec<Self> {
        Self::table().load::<Self>(db).unwrap_or_default()
    }

    pub fn insert(&self, db: &Conn) -> Result<Self, String> {
        Actor::get(Value::String(self.author.clone()), &db)?
            .insert(&db)
            .or_else(|e| Err(format!("Could not insert author. {}", e)))?;

        Object::get(Value::String(self.object.clone()), &db)?
            .insert(&db)
            .or_else(|e| Err(format!("Could not insert object. {}", e)))?;

        diesel::insert_into(Self::table())
            .values(self)
            .on_conflict_do_nothing()
            .execute(db)
            .or_else(|e| Err(format!("Could not insert activity. {}", e)))?;
        Self::read(self.id.clone(), db)
            .or_else(|e| Err(format!("Could not read inserted activity. {}", e)))
    }
    pub fn update(&self, db: &Conn) -> Result<Self, String> {
        diesel::update(Self::table().find(self.id.clone()))
            .set(self)
            .execute(db)
            .or_else(|e| Err(format!("Could not update activity. {}", e)))?;
        Self::read(self.id.clone(), db)
            .or_else(|e| Err(format!("Could not read updated activity. {}", e)))
    }
    pub fn remove(&self, db: &Conn) -> bool {
        diesel::delete(Self::table().find(self.id.clone()))
            .execute(db)
            .is_ok()
    }
}
