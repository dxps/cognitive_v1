use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextAttribute {
    //
    /// Its identifier.
    pub id: Id,

    /// Its name (inherited from its definition).
    pub name: String,

    /// Its value.
    pub value: String,

    /// Its definition id.
    pub def_id: Id,

    /// Its owner id.
    pub owner_id: Id,
}

impl TextAttribute {
    pub fn new(id: Id, name: String, value: String, def_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            def_id,
            owner_id,
        }
    }
}

impl Item for TextAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::TextAttribute
    }
}

impl From<AttributeDef> for TextAttribute {
    fn from(attr_def: AttributeDef) -> Self {
        Self::new(
            Id::default(),          // its id
            attr_def.name,          // its name
            attr_def.default_value, // its default value
            attr_def.id,            // its definition id
            Id::default(),          // owner id
        )
    }
}
