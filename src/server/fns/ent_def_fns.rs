use crate::domain::model::{EntityDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use indexmap::IndexMap;
use server_fn::codec::GetUrl;

/// List the entities definitions names, these being entities kinds.
#[server(endpoint = "admin/list_ent_defs_id_name", input = GetUrl)]
pub async fn list_entities_defs_id_name() -> Result<IndexMap<Id, String>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_def_mgmt().list_ids_names().await?;
    Ok(result)
}

/// List the entities definitions id and names, containing an attribute definition with the provided id.
#[server(endpoint = "admin/list_entity_defs_refs_by_attr_def_id", input = GetUrl)]
pub async fn list_entity_defs_refs_by_attr_def_id(attr_def_id: Id) -> Result<Vec<(Id, String)>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_def_mgmt().list_refs_by_attr_def_id(attr_def_id).await?;
    Ok(result)
}

/// List the entities definitions.
#[server(endpoint = "admin/list_ent_defs", input = GetUrl)]
pub async fn list_entities_defs() -> Result<IndexMap<Id, EntityDef>, ServerFnError> {
    let session: Session = extract().await?;
    let items = session.ent_def_mgmt().list().await?;
    Ok(IndexMap::from_iter(items.into_iter().map(|item| (item.id.clone(), item))))
}

/// Create an entity definition.
#[server(endpoint = "admin/create_ent_defs")]
pub async fn create_entity_def(item: EntityDef) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_def_mgmt().add(item).await;
    result.map_err(|e| e.into())
}

/// Get an entity definition.
#[server(endpoint = "admin/get_ent_def", input = GetUrl)]
pub async fn get_entity_def(id: Id) -> Result<Option<EntityDef>, ServerFnError> {
    let session: Session = extract().await?;
    let ent_def = session.ent_def_mgmt().get(&id).await;
    Ok(ent_def)
}

/// Update an entity definition.
#[server(endpoint = "admin/update_ent_defs")]
pub async fn update_entity_def(ent_def: EntityDef) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_def_mgmt().update(&ent_def).await;
    session
        .5
        .update_listing_addr_name(&ent_def.id, &ent_def.listing_attr_def_id)
        .await?;
    result.map_err(|e| e.into())
}

/// Remove an entity definition.
#[server(endpoint = "admin/remove_ent_defs")]
pub async fn remove_entity_def(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_def_mgmt().remove(&id).await;
    result.map_err(|e| e.into())
}
