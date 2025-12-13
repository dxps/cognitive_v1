use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// The definition of an `Entity`.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntityDef {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<AttributeDef>,
    pub listing_attr_def_id: Id,
}

impl EntityDef {
    pub fn new(id: Id, name: String, description: Option<String>, listing_attr_def_id: Id) -> Self {
        Self {
            id,
            name,
            description,
            attributes: vec![],
            listing_attr_def_id,
        }
    }

    pub fn new_with_attr_def_ids(
        id: Id,
        name: String,
        description: Option<String>,
        attributes: IndexMap<Id, String>,
        listing_attr_def_id: Id,
    ) -> Self {
        Self {
            id,
            name,
            description,
            listing_attr_def_id,
            attributes: attributes
                .iter()
                .map(|(id, name)| AttributeDef::new_with_id_name(id.clone(), name.clone()))
                .collect(),
        }
    }
}

impl Item for EntityDef {
    fn item_type(&self) -> ItemType {
        ItemType::EntityDef
    }
}
