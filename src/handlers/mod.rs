use serde::ser::{Serialize, SerializeMap, Serializer};

/// Activity handlers for the inbox route.
pub mod inbox;
/// Activity handlers for the outbox route.
pub mod outbox;

#[derive(Debug, Deserialize)]
/// Struct for representing an ordered collection.
pub struct OrderedCollection<T> {
    /// Unique, canonical URL for this collection.
    id: String,
    /// List of items within the collection.
    items: Vec<T>,
}

impl<T: Serialize> Serialize for OrderedCollection<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(4))?;
        s.serialize_entry("@context", "https://www.w3.org/ns/activitystreams")?;
        s.serialize_entry("id", &self.id)?;
        s.serialize_entry("type", "OrderedCollection")?;
        s.serialize_entry("orderedItems", &self.items)?;
        s.end()
    }
}
