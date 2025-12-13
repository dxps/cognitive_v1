use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// An instance of an attribute whose value type is boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    //
    /// Its identifier.
    pub id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: bool,

    /// Its definition id.
    pub def_id: Id,

    /// Its owner id.
    pub owner_id: Id,
}

impl BooleanAttribute {
    pub fn new(id: Id, name: String, value: bool, def_id: Id, owner_id: Id) -> Self {
        Self {
            id,
            name,
            value,
            def_id,
            owner_id,
        }
    }
}

impl Item for BooleanAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::BooleanAttribute
    }
}

impl From<AttributeDef> for BooleanAttribute {
    fn from(attr_def: AttributeDef) -> Self {
        let mut value = false;
        if !attr_def.default_value.trim().is_empty() {
            value = match attr_def.default_value.parse() {
                Ok(v) => v,
                Err(e) => {
                    log::error!(
                        "Failed to parse attr def id: '{}' default value: '{}' as boolean. Reason: '{}'.",
                        attr_def.id,
                        attr_def.default_value,
                        e,
                    );
                    false
                }
            };
        }
        Self::new(Id::default(), attr_def.name, value, attr_def.id, Id::default())
    }
}
