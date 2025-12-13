use std::collections::HashMap;

use crate::{
    domain::model::{AttributeValueType, BooleanAttribute, Entity, EntityDef, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, EntityForm, Nav, Select},
        pages::Name,
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use indexmap::IndexMap;

pub fn EntityNewPage() -> Element {
    //
    let mut ent_defs = use_signal::<Vec<EntityDef>>(|| Vec::new());
    let mut ent_kinds = use_signal::<IndexMap<Id, String>>(|| IndexMap::new());
    let selected_kind_id = use_signal(|| Id::default());
    let mut selected_kind_name = use_signal(|| Name::default());
    let mut listing_attr_def_id = use_signal(|| Id::default());
    let mut listing_attr_name = use_signal(|| Name::default());
    let listing_attr_value = use_signal(|| String::default());

    let mut text_attrs = use_signal::<HashMap<Id, TextAttribute>>(|| HashMap::new());
    let mut smallint_attrs = use_signal::<HashMap<Id, SmallintAttribute>>(|| HashMap::new());
    let mut int_attrs = use_signal::<HashMap<Id, IntegerAttribute>>(|| HashMap::new());
    let mut boolean_attrs = use_signal::<HashMap<Id, BooleanAttribute>>(|| HashMap::new());
    let mut attributes_order = use_signal::<Vec<(AttributeValueType, Id)>>(|| Vec::new());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut action_done = use_signal(|| false);

    use_future(move || async move {
        let ent_defs_list = UI_STATE.get_ent_defs_list().await;
        let mut ent_kinds_map = IndexMap::new();
        ent_defs_list.iter().for_each(|ent_def| {
            ent_kinds_map.insert(ent_def.id.clone(), ent_def.name.clone());
        });
        ent_kinds.set(ent_kinds_map);
        ent_defs.set(ent_defs_list);
    });

    use_memo(move || {
        let kind_id = selected_kind_id();
        log::debug!("[EntityNewPage] Changed selected kind_id: {:?}", kind_id);
        if kind_id.is_empty() {
            return;
        }
        selected_kind_name.set(ent_kinds().get(&kind_id).unwrap().clone());
        log::debug!(
            "[EntityNewPage] Loading attributes of entity def w/ id:'{}' from the global state ...",
            kind_id
        );
        if let Some(ent_def) = UI_STATE.get_ent_def_sync(&kind_id) {
            let mut txt_attrs = HashMap::new();
            let mut si_attrs = HashMap::new();
            let mut i_attrs = HashMap::new();
            let mut b_attrs = HashMap::new();
            let mut attrs_order = Vec::new();
            ent_def.attributes.into_iter().for_each(|attr_def| {
                if attr_def.id == ent_def.listing_attr_def_id {
                    listing_attr_def_id.set(attr_def.id.clone());
                    listing_attr_name.set(attr_def.name.clone());
                }
                match attr_def.value_type {
                    AttributeValueType::Text => {
                        attrs_order.push((AttributeValueType::Text, attr_def.id.clone()));
                        let attr = TextAttribute::from(attr_def);
                        txt_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::SmallInteger => {
                        attrs_order.push((AttributeValueType::SmallInteger, attr_def.id.clone()));
                        let attr = SmallintAttribute::from(attr_def);
                        si_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::Integer => {
                        attrs_order.push((AttributeValueType::Integer, attr_def.id.clone()));
                        let attr = IntegerAttribute::from(attr_def);
                        i_attrs.insert(attr.def_id.clone(), attr);
                    }
                    AttributeValueType::Boolean => {
                        attrs_order.push((AttributeValueType::Boolean, attr_def.id.clone()));
                        let attr = BooleanAttribute::from(attr_def);
                        b_attrs.insert(attr.def_id.clone(), attr);
                    }
                    _ => {}
                }
            });
            attributes_order.set(attrs_order);
            text_attrs.set(txt_attrs);
            smallint_attrs.set(si_attrs);
            int_attrs.set(i_attrs);
            boolean_attrs.set(b_attrs);
            log::debug!("[EntityNewPage] Loaded attributes from entity def w/ id:'{}'.", kind_id);
        } else {
            log::warn!("[EntityNewPage] Failed to get entity def w/ id:'{}'.", kind_id);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityListPage {},
                                "X"
                            }
                        }
                        div { class: "flex py-4",
                            p { class: "py-2 pr-4 text-gray-600 block", "Kind:" }
                            if !ent_defs().is_empty() {
                                Select {
                                    items: ent_kinds,
                                    selected_item_id: selected_kind_id,
                                    disabled: false,
                                }
                            }
                        }
                        if selected_kind_id().is_empty() {
                            p { class: "py-2 text-gray-500 block",
                                "You need to select its kind (entity definition) first."
                            }
                        } else {
                            EntityForm {
                                attributes_order,
                                text_attrs,
                                smallint_attrs,
                                int_attrs,
                                boolean_attrs,
                                action: Action::Edit,
                            }
                        }
                        div { class: "flex justify-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: selected_kind_id().is_empty(),
                                onclick: move |_| {
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::EntityListPage {});
                                        } else {
                                            handle_create_ent(
                                                    selected_kind_id(),
                                                    attributes_order(),
                                                    text_attrs().values().cloned().collect(),
                                                    smallint_attrs().values().cloned().collect(),
                                                    int_attrs().values().cloned().collect(),
                                                    boolean_attrs().values().cloned().collect(),
                                                    listing_attr_def_id(),
                                                    listing_attr_name(),
                                                    listing_attr_value(),
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
                        content: vec!["The entity has been successfully created.".into()],
                        action_handler: move |_| {
                            navigator().push(Route::EntityListPage {});
                        },
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: vec!["Failed to create the entity instance. Reason:".into(), err.unwrap()],
                        action_handler: move |_| {
                            err.set(None);
                            action_done.set(false);
                        },
                    }
                }
            }
        }
    }
}

async fn handle_create_ent(
    def_id: Id,
    attributes_order: Vec<(AttributeValueType, Id)>,
    text_attrs: Vec<TextAttribute>,
    smallint_attrs: Vec<SmallintAttribute>,
    int_attrs: Vec<IntegerAttribute>,
    boolean_attrs: Vec<BooleanAttribute>,
    listing_attr_def_id: Id,
    listing_attr_name: String,
    listing_attr_value: String,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) -> Option<Id> {
    //

    let ent = Entity::new(
        def_id,
        attributes_order,
        text_attrs,
        smallint_attrs,
        int_attrs,
        boolean_attrs,
        listing_attr_def_id,
        listing_attr_name,
        listing_attr_value,
    );

    log::debug!("[EntityNewPage] [handle_create_ent] Creating {:?} ...", ent);

    match crate::server::fns::create_entity(ent).await {
        Ok(id) => {
            action_done.set(true);
            err.set(None);
            Some(id)
        }
        Err(e) => {
            action_done.set(true);
            err.set(Some(e.to_string()));
            None
        }
    }
}
