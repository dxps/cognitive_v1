use serde::{Deserialize, Serialize};

pub trait Item {
    fn item_type(&self) -> ItemType;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Tag,
    AttributeDef,
    EntityDef,
    EntityLinkDef,
    Entity,
    EntityLink,
    TextAttribute,
    SmallintAttribute,
    IntegerAttribute,
    BooleanAttribute,
    Unknown,
}

impl ItemType {
    pub fn value(&self) -> String {
        match self {
            ItemType::Tag => "tag".to_string(),
            ItemType::AttributeDef => "atd".to_string(),
            ItemType::EntityDef => "end".to_string(),
            ItemType::EntityLinkDef => "eld".to_string(),
            ItemType::Entity => "eni".to_string(),
            ItemType::EntityLink => "enl".to_string(),
            ItemType::TextAttribute => "tea".to_string(),
            ItemType::SmallintAttribute => "sma".to_string(),
            ItemType::IntegerAttribute => "ina".to_string(),
            ItemType::BooleanAttribute => "boa".to_string(),
            ItemType::Unknown => "unk".to_string(),
        }
    }
}

impl From<&str> for ItemType {
    fn from(value: &str) -> Self {
        match value {
            "tag" => ItemType::Tag,
            "atd" => ItemType::AttributeDef,
            "end" => ItemType::EntityDef,
            "eld" => ItemType::EntityLinkDef,
            "eni" => ItemType::Entity,
            "enl" => ItemType::EntityLink,
            "tea" => ItemType::TextAttribute,
            "boa" => ItemType::BooleanAttribute,
            "sma" => ItemType::SmallintAttribute,
            "ina" => ItemType::IntegerAttribute,
            "unk" => ItemType::Unknown,
            _ => ItemType::Tag,
        }
    }
}
