use std::time::SystemTime;

use serde_json::Value;

use diesel::associations::HasTable;
use diesel::{QueryDsl, RunQueryDsl};

use super::actors::Actor;
use super::fetch;

use crate::connection::Conn;
use crate::schema::objects;

/// A model for a stored object.
#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct Object {
    /// The id of the object as a URL to the original activity.
    pub id: String,
    /// The type of the object.
    pub objtype: String,
    /// Foreign key to the author of the object, in the form of a URL to original Actor object.
    pub author: String,
    /// Time object was published if available.
    pub published: Option<SystemTime>,
    /// The contents of the object.
    pub contents: Option<serde_json::Value>,
}

impl Object {
    /// Obtains an Object. By parsing contents or reading from database.
    ///
    /// If contents is an JSON struct, the value is parsed into a Object struct.
    /// If contents is a string, the value is read from the database or retrieved based on the ID.
    ///
    /// Does not insert Object, only creates struct.
    pub async fn get(contents: Value, db: &Conn) -> Result<Self, String> {
        let contents = match contents {
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

        // Parses Object struct from JSON struct.
        let id = contents
            .get("id")
            .ok_or_else(|| "No type found in object.".to_string())?
            .as_str()
            .ok_or("No object id")?
            .to_string();
        let objtype = contents
            .get("type")
            .ok_or_else(|| "No type found in object.".to_string())?
            .as_str()
            .ok_or("No type field found in object")?
            .to_string();
        let actor = Actor::get(
            contents
                .get("attributedTo")
                .ok_or_else(|| "No actor found in object.".to_string())?
                .clone(),
            &db,
        )
        .await?;

        Ok(Object {
            id,
            objtype,
            author: actor.id,
            published: None,
            contents: Some(Value::Object(contents)),
        })
    }

    /// Reads an Object from the database.
    pub fn read(id: String, db: &Conn) -> Result<Self, String> {
        Self::table()
            .find(id)
            .first(db)
            .map_err(|e| format!("Could not read object. {}", e))
    }

    /// Reads all Objects from the database.
    pub fn list(db: &Conn) -> Vec<Self> {
        Self::table().load::<Self>(db).unwrap_or_default()
    }

    /// Inserts an Object into the database.
    ///
    /// Also attempts to obtain and insert the Actor, if it does not already exist.
    pub async fn insert(&self, db: &Conn) -> Result<Self, String> {
        // Attempt to insert author.
        Actor::get(Value::String(self.author.clone()), &db)
            .await?
            .insert(&db)
            .await
            .map_err(|e| format!("Could not insert author. {}", e))?;

        // Insert Object into database.
        diesel::insert_into(Self::table())
            .values(self)
            .on_conflict_do_nothing()
            .execute(db)
            .map_err(|e| format!("Could not insert object. {}", e))?;

        // Return Object from database using id.
        Self::read(self.id.clone(), db)
            .map_err(|e| format!("Could not read inserted object. {}", e))
    }

    /// Updates an Object in the database.
    pub fn update(&self, db: &Conn) -> Result<Self, String> {
        diesel::update(Self::table().find(self.id.clone()))
            .set(self)
            .execute(db)
            .map_err(|e| format!("Could not update object. {}", e))?;

        // Return updated Object from database using id.
        Self::read(self.id.clone(), db).map_err(|e| format!("Could not read updated object. {}", e))
    }

    /// Removes an Object from the database.
    pub fn remove(&self, db: &Conn) -> bool {
        diesel::delete(Self::table().find(self.id.clone()))
            .execute(db)
            .is_ok()
    }
}
