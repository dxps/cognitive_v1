use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type integer.\
/// Its value ranges from -2147483648 to +2147483647.\
/// Stored in PostgreSQL in an `int4` data type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerAttribute {
    //
    /// Its identifier.
    pub id: Id,

    /// Its name (inherited from its definition).
    pub name: String,

    /// Its value.
    pub value: i32,

    /// Its definition id.
    pub def_id: Id,

    /// Its owner id.
    pub owner_id: Id,
}

impl IntegerAttribute {
    pub fn new(id: Id, name: String, value: i32, def_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            def_id,
            owner_id,
        }
    }
}

impl Item for IntegerAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}

impl From<AttributeDef> for IntegerAttribute {
    fn from(attr_def: AttributeDef) -> Self {
        let mut value: i32 = 0;
        if !attr_def.default_value.trim().is_empty() {
            value = match attr_def.default_value.parse() {
                Ok(v) => v,
                Err(e) => {
                    log::error!(
                        "Failed to parse attr def id: '{}' default value: '{}' as i32. Reason: '{}'.",
                        attr_def.id,
                        attr_def.default_value,
                        e,
                    );
                    0
                }
            }
        }
        Self::new(Id::default(), attr_def.name, value, attr_def.id, Id::default())
    }
}
