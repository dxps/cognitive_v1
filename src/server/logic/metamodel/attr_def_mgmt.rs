use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, Id},
    server::{AppResult, AttributeDefRepo},
};

pub struct AttributeDefMgmt {
    attr_repo: Arc<AttributeDefRepo>,
}

impl AttributeDefMgmt {
    //
    pub fn new(attr_repo: Arc<AttributeDefRepo>) -> Self {
        Self { attr_repo }
    }

    pub async fn get(&self, id: &Id) -> Option<AttributeDef> {
        //
        self.attr_repo.get(id).await
    }

    pub async fn list(&self) -> Vec<AttributeDef> {
        //
        self.attr_repo.list(None).await
    }

    /// Add a new attribute definition. It returns the id of the stored entry.
    pub async fn add(&self, mut item: AttributeDef) -> AppResult<Id> {
        //
        let id = Id::new();
        log::debug!("Adding {:?} ...", item);
        item.id = id.clone();
        self.attr_repo.add(&item).await.map(|_| id)
    }

    /// Update an existing attribute definition.
    pub async fn update(&self, item: &AttributeDef) -> AppResult<()> {
        //
        self.attr_repo.update(item).await
    }

    /// Remove an existing attribute definition.
    pub async fn remove(&self, id: Id) -> AppResult<()> {
        //
        self.attr_repo.remove(&id).await
    }
}
