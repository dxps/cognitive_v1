use crate::{
    domain::model::{EntityLinkDef, Id},
    server::{AppResult, EntityLinkDefRepo},
};
use std::sync::Arc;

pub struct EntityLinkDefMgmt {
    ent_link_def_repo: Arc<EntityLinkDefRepo>,
}

impl EntityLinkDefMgmt {
    //
    pub fn new(ent_link_def_repo: Arc<EntityLinkDefRepo>) -> Self {
        Self { ent_link_def_repo }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityLinkDef>> {
        self.ent_link_def_repo.list().await
    }

    pub async fn add(&self, mut item: EntityLinkDef) -> AppResult<Id> {
        item.id = Id::new();
        self.ent_link_def_repo.add(&item).await?;
        Ok(item.id)
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<EntityLinkDef>> {
        //
        self.ent_link_def_repo.get(id).await
    }

    pub async fn update(&self, item: &EntityLinkDef) -> AppResult<()> {
        //
        self.ent_link_def_repo.update(item).await
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        self.ent_link_def_repo.remove(id).await
    }
}
