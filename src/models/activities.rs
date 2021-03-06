use chrono::DateTime;
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
    pub published: SystemTime,
    /// The id of the associated object, as a URL to the original object.
    pub object: String,
    /// The contents of the activity.
    pub contents: Value,
}

impl Activity {
    /// Obtains an Activity. By parsing contents or reading from database.
    ///
    /// If contents is an JSON struct, the value is parsed into a Activity struct.
    /// If contents is a string, the value is read from the database or retrieved based on the ID.
    ///
    /// Does not insert Activity, only creates struct.
    pub async fn get(contents: Value, db: &Conn) -> Result<Self, String> {
        let mut contents = match contents {
            Value::String(id) => match Self::read(id.clone(), db) {
                Ok(object) => return Ok(object),
                _ => fetch(id)
                    .await?
                    .as_object()
                    .ok_or_else(|| "Invalid activity reference.".to_string())?
                    .to_owned(),
            },
            Value::Object(contents) => contents,
            _ => return Err("Invalid activity reference.".to_string()),
        };

        let actor = Actor::get(
            contents.get("actor").ok_or_else(|| "No actor.")?.clone(),
            &db,
        )
        .await?;
        let object = Object::get(
            contents.get("object").ok_or_else(|| "No object.")?.clone(),
            &db,
        )
        .await?;

        // Replace inline Object with Object ID
        contents.insert("object".to_string(), Value::String(object.id.clone()));

        // Parses Activity struct from JSON struct.
        let activity_id = contents
            .get("id")
            .ok_or_else(|| "activity id.")?
            .as_str()
            .ok_or("No activity id")?;

        let acttype = contents["type"]
            .as_str()
            .ok_or("No type field found in activity")?
            .to_string();

        let published = contents
            .get("published")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .ok_or_else(|| "No published date.")?;

        Ok(Activity {
            id: activity_id.to_string(),
            acttype,
            author: actor.id,
            object: object.id,
            contents: Value::Object(contents.to_owned()),
            published: published.into(),
        })
    }

    /// Reads an Activity from the database.
    pub fn read(id: String, db: &Conn) -> Result<Self, String> {
        Self::table()
            .find(id)
            .first(db)
            .map_err(|e| format!("Could not read activity. {}", e))
    }

    /// Reads all Activities from the database.
    pub fn list(db: &Conn) -> Vec<Self> {
        Self::table().load::<Self>(db).unwrap_or_default()
    }

    /// Inserts an Activity into the database.
    ///
    /// Also attempts to obtain and insert the Actor and Object, if they do not already exist.
    pub async fn insert(&self, db: &Conn) -> Result<Self, String> {
        Actor::get(Value::String(self.author.clone()), &db)
            .await?
            .insert(&db)
            .await
            .map_err(|e| format!("Could not insert author. {}", e))?;

        Object::get(Value::String(self.object.clone()), &db)
            .await?
            .insert(&db)
            .await
            .map_err(|e| format!("Could not insert object. {}", e))?;

        // Insert Object into database.
        diesel::insert_into(Self::table())
            .values(self)
            .on_conflict_do_nothing()
            .execute(db)
            .map_err(|e| format!("Could not insert activity. {}", e))?;

        // Return Activity from database using id.
        Self::read(self.id.clone(), db)
            .map_err(|e| format!("Could not read inserted activity. {}", e))
    }

    /// Updates an Object in the database.
    pub fn update(&self, db: &Conn) -> Result<Self, String> {
        diesel::update(Self::table().find(self.id.clone()))
            .set(self)
            .execute(db)
            .map_err(|e| format!("Could not update activity. {}", e))?;

        // Return updated Activity from database using id.
        Self::read(self.id.clone(), db)
            .map_err(|e| format!("Could not read updated activity. {}", e))
    }

    /// Removes an Object from the database.
    pub fn remove(&self, db: &Conn) -> bool {
        diesel::delete(Self::table().find(self.id.clone()))
            .execute(db)
            .is_ok()
    }
}
