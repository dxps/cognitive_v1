use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

use crate::domain::model::{AttributeDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

/// List the attribute definitions.
#[server(endpoint = "admin/list_attr_defs", input = GetUrl)]
pub async fn list_attribute_defs() -> Result<Vec<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_defs = session.attr_def_mgmt().list().await;
    Ok(attr_defs)
}

/// Get an attribute definitions.
#[server(endpoint = "admin/get_attr_def", input = GetUrl)]
pub async fn get_attribute_def(id: Id) -> Result<Option<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_def = session.attr_def_mgmt().get(&id).await;
    Ok(attr_def)
}

/// Create an attribute definition.
#[server(endpoint = "admin/create_attr_def")]
pub async fn create_attribute_def(item: AttributeDef) -> Result<Id, ServerFnError> {
    //
    let session: Session = extract().await?;
    session.attr_def_mgmt().add(item).await.map(|id| Ok(id))?
}

/// Update an attribute definition.
#[server(endpoint = "admin/update_attr_def")]
pub async fn update_attribute_def(attr_def: AttributeDef) -> Result<(), ServerFnError> {
    //
    log::debug!("Updating attribute def: {:?}", attr_def);
    let session: Session = extract().await?;
    if let Err(e) = session
        .attr_def_mgmt()
        .update(&attr_def)
        .await
        .map(|_| Ok::<_, ServerFnError>(()))?
    {
        return Err(e);
    };
    session
        .5
        .update_listing_attr_name_by_attr_def_id(&attr_def.id, &attr_def.name)
        .await
        .map(|_| Ok(()))?
}

/// Remove an attribute definition.
#[server(endpoint = "remove_attr_def")]
pub async fn remove_attr_def(id: Id) -> Result<(), ServerFnError> {
    //
    let session: Session = extract().await?;
    session.attr_def_mgmt().remove(id).await.map(|_| Ok(()))?
}
