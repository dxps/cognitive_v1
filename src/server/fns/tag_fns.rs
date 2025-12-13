use crate::domain::model::{Id, Tag};
use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

#[cfg(feature = "server")]
use crate::server::Session;

#[server(endpoint = "get_tags", input = GetUrl)]
pub async fn get_tags() -> Result<Vec<Tag>, ServerFnError> {
    //
    let session: Session = extract().await?;
    let tags = session.tag_mgmt().list().await?;
    Ok(tags)
}

#[server(endpoint = "create_tag")]
pub async fn create_tag(name: String, description: Option<String>) -> Result<Id, ServerFnError> {
    //
    let session: Session = extract().await?;
    let tags = session.tag_mgmt().add(name, description).await?;
    Ok(tags)
}

#[server(endpoint = "update_tag")]
pub async fn update_tag(tag: Tag) -> Result<(), ServerFnError> {
    //
    let session: Session = extract().await?;
    session.tag_mgmt().update(tag).await.map(|_| Ok(()))?
}

#[server(endpoint = "remove_tag")]
pub async fn remove_tag(id: Id) -> Result<(), ServerFnError> {
    //
    let session: Session = extract().await?;
    session.tag_mgmt().remove(id).await.map(|_| Ok(()))?
}
