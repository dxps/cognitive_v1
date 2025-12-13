use crate::{
    domain::model::{
        AttributeValueType, BooleanAttribute, EntityLink, EntityLinkDef, Id, IntegerAttribute, SmallintAttribute, TextAttribute,
    },
    server::fns::list_entities_by_def_id,
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, EntityLinkForm, Nav, Select},
        pages::Name,
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use indexmap::IndexMap;

pub fn EntityLinkNewPage() -> Element {
    //
    let mut ent_link_defs = use_signal::<Vec<EntityLinkDef>>(|| Vec::new());
    let mut ent_link_kinds = use_signal::<IndexMap<Id, Name>>(|| IndexMap::new());
    let selected_kind_id = use_signal(|| Id::default());
    let mut selected_kind_name = use_signal(|| Name::default());

    let mut def_source_entity_id = use_signal(|| Id::default());
    let mut def_target_entity_id = use_signal(|| Id::default());

    let source_entity_id = use_signal(|| Id::default());
    let target_entity_id = use_signal(|| Id::default());

    let mut source_entities_id_name = use_signal(|| IndexMap::<Id, Name>::new());
    let mut target_entities_id_name = use_signal(|| IndexMap::<Id, Name>::new());

    let mut text_attrs = use_signal::<IndexMap<Id, TextAttribute>>(|| IndexMap::new());
    let mut smallint_attrs = use_signal::<IndexMap<Id, SmallintAttribute>>(|| IndexMap::new());
    let mut int_attrs = use_signal::<IndexMap<Id, IntegerAttribute>>(|| IndexMap::new());
    let mut boolean_attrs = use_signal::<IndexMap<Id, BooleanAttribute>>(|| IndexMap::new());

    let create_btn_disabled =
        use_memo(move || selected_kind_name().is_empty() || source_entity_id().is_empty() || target_entity_id().is_empty());
    let err: Signal<Option<String>> = use_signal(|| None);
    let action_done = use_signal(|| false);

    use_future(move || async move {
        let ent_link_defs_list = UI_STATE.get_ent_link_def_list().await;
        let mut id_kind_map = IndexMap::new();
        ent_link_defs_list.iter().for_each(|ent_def| {
            id_kind_map.insert(ent_def.id.clone(), ent_def.name.clone());
        });
        ent_link_kinds.set(id_kind_map);
        ent_link_defs.set(ent_link_defs_list);
    });

    use_memo(move || {
        let kind_id = selected_kind_id();
        log::debug!("[EntityLinkNewPage] Changed selected_kind_id: {:?}", kind_id);
        if kind_id.is_empty() {
            return;
        }
        selected_kind_name.set(ent_link_kinds().get(&kind_id).unwrap().clone());

        if let Some(ent_link_def) = UI_STATE.get_ent_link_def_sync(&kind_id) {
            def_source_entity_id.set(ent_link_def.source_entity_def_id.clone());
            def_target_entity_id.set(ent_link_def.target_entity_def_id.clone());

            let def_id = kind_id.clone();
            spawn(async move {
                log::debug!("[EntityLinkNewPage] Loading source entities by def id:'{}' ...", def_id);
                match list_entities_by_def_id(ent_link_def.source_entity_def_id).await {
                    Ok(source_entities) => {
                        let mut id_name_map = IndexMap::new();
                        for ent in source_entities {
                            id_name_map.insert(ent.id, format!("{}: {}", ent.listing_attr_name, ent.listing_attr_value));
                        }
                        source_entities_id_name.set(id_name_map);
                    }
                    Err(e) => {
                        log::error!("[EntityLinkNewPage] Error loading source entities by def id:'{}': {}", def_id, e);
                    }
                }
                match list_entities_by_def_id(ent_link_def.target_entity_def_id).await {
                    Ok(target_entities) => {
                        let mut id_name_map = IndexMap::new();
                        for ent in target_entities {
                            id_name_map.insert(ent.id, format!("{}: {}", ent.listing_attr_name, ent.listing_attr_value));
                        }
                        target_entities_id_name.set(id_name_map);
                    }
                    Err(e) => {
                        log::error!("[EntityLinkNewPage] Error loading target entities by def id:'{}': {}", def_id, e);
                    }
                }
            });

            if ent_link_def.attributes.is_some() {
                let attr_defs = ent_link_def.attributes.unwrap();
                log::debug!(
                    "[EntityLinkNewPage] Entity link def id:'{}' has {} attributes.",
                    kind_id,
                    attr_defs.len()
                );
                let mut txt_attrs = IndexMap::new();
                let mut si_attrs = IndexMap::new();
                let mut i_attrs = IndexMap::new();
                let mut b_attrs = IndexMap::new();
                attr_defs.into_iter().for_each(|attr_def| match attr_def.value_type {
                    AttributeValueType::Text => {
                        let attr = TextAttribute::from(attr_def);
                        txt_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::SmallInteger => {
                        let attr = SmallintAttribute::from(attr_def);
                        si_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::Integer => {
                        let attr = IntegerAttribute::from(attr_def);
                        i_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::Boolean => {
                        let attr = BooleanAttribute::from(attr_def);
                        b_attrs.insert(attr.def_id.clone(), attr);
                    }
                    _ => {}
                });
                text_attrs.set(txt_attrs);
                smallint_attrs.set(si_attrs);
                int_attrs.set(i_attrs);
                boolean_attrs.set(b_attrs);
                log::debug!("[EntityLinkNewPage] Loaded attributes from entity link def id:'{}'", kind_id);
            } else {
                log::debug!("[EntityLinkNewPage] Entity link def id:'{}' has no attributes.", kind_id);
            }
        } else {
            log::warn!("[EntityLinkNewPage] Failed to get entity link def id:'{}'", kind_id);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Link"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkListPage {},
                                "X"
                            }
                        }
                        if selected_kind_id().is_empty() {
                            div {
                                p { class: "pb-4 text-gray-500 block",
                                    "Select the kind of entity link to create."
                                }
                            }
                            div { class: "flex",
                                p { class: "py-2 pr-4 text-gray-600 block", "Kind:" }
                                if !ent_link_defs().is_empty() {
                                    Select {
                                        items: ent_link_kinds,
                                        selected_item_id: selected_kind_id,
                                    }
                                }
                            }
                        } else {
                            EntityLinkForm {
                                source_entity_id,
                                source_entities_id_name,
                                target_entity_id,
                                target_entities_id_name,
                                text_attrs,
                                smallint_attrs,
                                int_attrs,
                                boolean_attrs,
                                action: Action::Edit,
                            }
                        }
                        div { class: "grid justify-items-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: create_btn_disabled(),
                                onclick: move |_| {
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::EntityListPage {});
                                        } else {
                                            handle_create_ent_link(
                                                    selected_kind_id(),
                                                    source_entity_id(),
                                                    target_entity_id(),
                                                    text_attrs().values().cloned().collect(),
                                                    smallint_attrs().values().cloned().collect(),
                                                    int_attrs().values().cloned().collect(),
                                                    boolean_attrs().values().cloned().collect(),
                                                    action_done,
                                                    err,
                                                )
                                                .await;
                                        }
                                    }
                                },
                                if action_done() {
                                    "Close"
                                } else {
                                    "Create"
                                }
                            }
                        }
                    }
                }
            }
            if action_done() {
                if err().is_none() {
                    AcknowledgeModal {
                        title: "Confirmation",
                        content: vec!["The entity link has been successfully created.".into()],
                        action_handler: move |_| {
                            navigator().push(Route::EntityLinkListPage {});
                        },
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: vec!["Failed to create the entity link. Reason:".into(), err.unwrap()],
                        action_handler: move |_| {
                            navigator().push(Route::EntityListPage {});
                        },
                    }
                }
            }
        }
    }
}

async fn handle_create_ent_link(
    def_id: Id,
    source_entity_id: Id,
    target_entity_id: Id,
    text_attrs: Vec<TextAttribute>,
    smallint_attrs: Vec<SmallintAttribute>,
    int_attrs: Vec<IntegerAttribute>,
    boolean_attrs: Vec<BooleanAttribute>,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    //
    let ent_link = EntityLink::from(
        def_id,
        source_entity_id,
        target_entity_id,
        text_attrs,
        smallint_attrs,
        int_attrs,
        boolean_attrs,
    );

    log::debug!("Creating the entity link {:?} ...", ent_link);

    match crate::server::fns::create_entity_link(ent_link).await {
        Ok(id) => {
            saved.set(true);
            err.set(None);
            Some(id)
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
            None
        }
    }
}
