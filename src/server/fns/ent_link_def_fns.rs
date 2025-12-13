use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

use crate::domain::model::{EntityLinkDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

/// List the entity link definitions.
#[server(endpoint = "admin/list_ent_link_defs", input = GetUrl)]
pub async fn list_entity_link_defs() -> Result<Vec<EntityLinkDef>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_def_mgmt().list().await;
    result.map_err(|e| e.into())
}

/// Create an entity link definition.
#[server(endpoint = "admin/create_ent_link_def")]
pub async fn create_entity_link_def(item: EntityLinkDef) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_def_mgmt().add(item).await;
    result.map_err(|e| e.into())
}

/// Get an entity link definition.
#[server(endpoint = "admin/get_ent_link_def", input = GetUrl)]
pub async fn get_entity_link_def(id: Id) -> Result<Option<EntityLinkDef>, ServerFnError> {
    let session: Session = extract().await?;
    let ent_link_def = session.ent_link_def_mgmt().get(&id).await?;
    Ok(ent_link_def)
}

/// Update an entity link definition.
#[server(endpoint = "admin/update_ent_link_def")]
pub async fn update_entity_link_def(ent_link_def: EntityLinkDef) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_def_mgmt().update(&ent_link_def).await;
    result.map_err(|e| e.into())
}

/// Remove an entity link definition.
#[server(endpoint = "admin/remove_ent_link_def")]
pub async fn remove_entity_link_def(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_def_mgmt().remove(&id).await;
    result.map_err(|e| e.into())
}
