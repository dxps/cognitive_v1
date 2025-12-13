use dioxus_fullstack::prelude::*;
use indexmap::IndexMap;
use server_fn::codec::{GetUrl, PostUrl};

use crate::{
    domain::model::{EntityLink, Id},
    ui::pages::Name,
};

#[cfg(feature = "server")]
use crate::server::Session;

use super::get_entity_link_def;

/// List the entity links.
#[server(endpoint = "admin/list_ent_links", input = GetUrl)]
pub async fn list_entity_links() -> Result<Vec<EntityLink>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().list().await;
    result.map_err(|e| e.into())
}

/// List the entity links by their definition id.
#[server(endpoint = "admin/list_ent_links_by_def_id/:id", input = GetUrl)]
pub async fn list_entity_links_by_def_id(id: Id) -> Result<Vec<EntityLink>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().list_by_def_id(&id).await;
    result.map_err(|e| e.into())
}

/// List the entity links as references (containing only the id and name) by their definition id.
#[server(endpoint = "admin/list_ent_links_refs_by_def_id/:id", input = GetUrl)]
pub async fn list_entity_links_refs_by_def_id(id: Id) -> Result<Vec<(Id, Name)>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().list_by_def_id(&id).await;
    result
        .map(|items| {
            items
                .into_iter()
                .map(|ent_link| (ent_link.id.clone(), format!("{} (id: {})", ent_link.kind, ent_link.id)))
                .collect()
        })
        .map_err(|e| e.into())
}

/// Create an entity link.
#[server(endpoint = "admin/create_ent_link", input = PostUrl)]
pub async fn create_entity_link(item: EntityLink) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().add(item).await;
    result.map_err(|e| e.into())
}

/// Get an entity link.
#[server(endpoint = "admin/get_ent_link", input = GetUrl)]
pub async fn get_entity_link(id: Id) -> Result<Option<EntityLink>, ServerFnError> {
    let session: Session = extract().await?;
    let ent_link_opt = session.ent_link_mgmt().get(&id).await?;
    Ok(ent_link_opt)
}

/// Get all the details needed for presenting an entity link in the page.\
/// It returns the entity link, maps of source_entities_id_name and target_entities_id_name.
#[server(endpoint = "admin/get_ent_link_page_data", input = GetUrl)]
pub async fn get_entity_link_page_data(id: Id) -> Result<Option<(EntityLink, IndexMap<Id, Name>, IndexMap<Id, Name>)>, ServerFnError> {
    //
    let session: Session = extract().await?;
    let ent_link = session.ent_link_mgmt().get(&id).await?;

    if ent_link.is_none() {
        return Ok(None);
    }
    let ent_link = ent_link.unwrap();
    let mut source_entities_id_name = IndexMap::<Id, Name>::new();
    let mut target_entities_id_name = IndexMap::<Id, Name>::new();

    match get_entity_link_def(ent_link.def_id.clone()).await {
        Result::Ok(eld_opt) => {
            if let Some(eld) = eld_opt {
                match session.ent_mgmt().list_by_def_id(&eld.source_entity_def_id).await {
                    Ok(source_entities) => {
                        for ent in source_entities {
                            source_entities_id_name.insert(ent.id, format!("{}: {}", ent.listing_attr_name, ent.listing_attr_value));
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "[EntityLinkNewPage] Error loading source entities by def id:'{}': {}",
                            eld.source_entity_def_id,
                            e
                        );
                    }
                }
                match session.ent_mgmt().list_by_def_id(&eld.target_entity_def_id).await {
                    Ok(target_entities) => {
                        for ent in target_entities {
                            target_entities_id_name.insert(ent.id, format!("{}: {}", ent.listing_attr_name, ent.listing_attr_value));
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "[EntityLinkNewPage] Error loading target entities by def id:'{}': {}",
                            eld.source_entity_def_id,
                            e
                        );
                    }
                }
            }
        }
        Err(e) => {
            log::error!(
                "[EntityLinkPage] Failed to get entity link def w/ id: '{}'. Reason: '{}'.",
                ent_link.def_id,
                e
            );
            return Err(e);
        }
    }

    Ok(Some((ent_link, source_entities_id_name, target_entities_id_name)))
}

/// Update an entity link.
#[server(endpoint = "admin/update_ent_link")]
pub async fn update_entity_link(ent_link_def: EntityLink) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().update(&ent_link_def).await;
    result.map_err(|e| e.into())
}

// /// Remove an entity link.
#[server(endpoint = "admin/remove_ent_link", input = PostUrl)]
pub async fn remove_entity_link(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.ent_link_mgmt().remove(&id).await;
    result.map_err(|e| e.into())
}
