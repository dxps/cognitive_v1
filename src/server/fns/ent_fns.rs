use crate::{
    domain::model::{Entity, Id},
    ui::pages::Name,
};

#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

/// List the entities instances.
#[server(endpoint = "admin/list_ents", input = GetUrl)]
pub async fn list_entities() -> Result<Vec<Entity>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().list().await;
    result.map_err(|e| e.into())
}

/// Create an entity instance.
#[server(endpoint = "admin/create_ent")]
pub async fn create_entity(item: Entity) -> Result<Id, ServerFnError> {
    log::debug!("[create_entity (fn)] {:?}.", item);
    let session: Session = extract().await?;
    let result = session.ent_mgmt().add(item).await;
    result.map_err(|e| e.into())
}

/// Get an entity instance.
#[server(endpoint = "admin/get_ent", input = GetUrl)]
pub async fn get_entity(id: Id) -> Result<Option<Entity>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().get(&id).await;
    result.map_err(|e| e.into())
}

/// List the entities with the same definition.
#[server(endpoint = "admin/list_ents_by_def_id/:id", input = GetUrl)]
pub async fn list_entities_by_def_id(id: Id) -> Result<Vec<Entity>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().list_by_def_id(&id).await;
    result.map_err(|e| e.into())
}

/// List the entities refs (id and name) with the same definition.
#[server(endpoint = "admin/list_ents_refs_by_def_id/:id", input = GetUrl)]
pub async fn list_entities_refs_by_def_id(id: Id) -> Result<Vec<(Id, Name)>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().list_refs_by_def_id(&id).await?;
    let result = result
        .into_iter()
        .map(|(id, name)| (id.clone(), format!("{} (id: {})", name, id)))
        .collect::<Vec<(Id, Name)>>();
    Ok(result)
}

/// Update an entity instance.
#[server(endpoint = "admin/update_ent")]
pub async fn update_entity(ent: Entity) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().update(&ent).await;
    result.map_err(|e| e.into())
}

/// Remove an entity instance.
#[server(endpoint = "admin/remove_ent")]
pub async fn remove_entity(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_mgmt().remove(&id).await;
    result.map_err(|e| e.into())
}
