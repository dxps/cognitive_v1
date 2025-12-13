use indexmap::IndexMap;

///!
///! Commonly used logic.
///!
use crate::domain::model::Id;
use crate::server::fns::list_attribute_defs;

/// Fetch all attribute definitions and return a map of their id and (name, description).
pub async fn fetch_all_attr_defs() -> IndexMap<Id, (String, Option<String>)> {
    //
    let mut entries = IndexMap::new();
    if let Ok(attr_defs) = list_attribute_defs().await {
        attr_defs.iter().for_each(|attr_def| {
            entries.insert(attr_def.id.clone(), (attr_def.name.clone(), attr_def.description.clone()));
        });
    }
    entries
}
