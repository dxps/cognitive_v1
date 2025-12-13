use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
}

impl Tag {
    pub fn new(id: Id, name: String, description: Option<String>) -> Self {
        Self { id, name, description }
    }
}
