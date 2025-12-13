use crate::{
    domain::model::{Id, Tag},
    server::{AppResult, TagsRepo},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct TagMgmt {
    tag_repo: Arc<TagsRepo>,
}

impl TagMgmt {
    //
    pub fn new(tag_repo: Arc<TagsRepo>) -> Self {
        Self { tag_repo }
    }

    pub async fn get(&self, id: String) -> AppResult<Option<Tag>> {
        //
        self.tag_repo.get(id).await
    }

    pub async fn list(&self) -> AppResult<Vec<Tag>> {
        //
        self.tag_repo.list(None).await
    }

    pub async fn add(&self, name: String, description: Option<String>) -> AppResult<Id> {
        //
        let id = Id::new();
        let tag = Tag::new(id.clone(), name, description);
        _ = self.tag_repo.add(tag).await;
        Ok(id)
    }

    pub async fn update(&self, tag: Tag) -> AppResult<()> {
        //
        self.tag_repo.update(tag).await
    }

    pub async fn remove(&self, id: Id) -> AppResult<()> {
        //
        self.tag_repo.remove(id).await
    }
}
