use crate::{
    domain::model::{EntityLink, Id},
    server::{AppResult, EntityLinkRepo},
};
use std::sync::Arc;

pub struct EntityLinkMgmt {
    repo: Arc<EntityLinkRepo>,
}

impl EntityLinkMgmt {
    //
    pub fn new(repo: Arc<EntityLinkRepo>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityLink>> {
        self.repo.list(None).await
    }

    pub async fn list_by_def_id(&self, def_id: &Id) -> AppResult<Vec<EntityLink>> {
        self.repo.list_by_def_id(def_id).await
    }

    pub async fn add(&self, mut ent_link: EntityLink) -> AppResult<Id> {
        ent_link.id = Id::new();
        self.repo.add(&ent_link).await?;
        Ok(ent_link.id)
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<EntityLink>> {
        self.repo.get(id).await
    }

    pub async fn update(&self, item: &EntityLink) -> AppResult<()> {
        self.repo.update(item).await
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        self.repo.remove(id).await
    }
}
