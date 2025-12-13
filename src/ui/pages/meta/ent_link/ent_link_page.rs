use crate::{
    domain::model::{BooleanAttribute, EntityLink, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    server::fns::{get_entity_link, get_entity_link_def, list_entities_by_def_id, remove_entity_link, update_entity_link},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, EntityLinkForm, Nav},
        pages::Name,
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(PartialEq, Props, Clone)]
pub struct EntityLinkPageProps {
    id: Id,
}

#[component]
pub fn EntityLinkPage(props: EntityLinkPageProps) -> Element {
    //
    let id = use_signal(|| props.id);

    let kind = use_signal(|| Name::default());
    let kind_id = use_signal(|| Id::default());

    let source_entity_id = use_signal(|| Id::default());
    let target_entity_id = use_signal(|| Id::default());
    let source_entity_def_id = use_signal(|| Id::default());
    let target_entity_def_id = use_signal(|| Id::default());

    let source_entities_id_name = use_signal(|| IndexMap::<Id, Name>::new());
    let target_entities_id_name = use_signal(|| IndexMap::<Id, Name>::new());

    let text_attrs = use_signal::<IndexMap<Id, TextAttribute>>(|| IndexMap::new());
    let smallint_attrs = use_signal::<IndexMap<Id, SmallintAttribute>>(|| IndexMap::new());
    let int_attrs = use_signal::<IndexMap<Id, IntegerAttribute>>(|| IndexMap::new());
    let boolean_attrs = use_signal::<IndexMap<Id, BooleanAttribute>>(|| IndexMap::new());

    let update_btn_disabled = use_memo(move || source_entity_def_id().is_empty() || target_entity_def_id().is_empty());
    let mut show_delete_confirm = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let mut action_done = use_signal(|| false);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        init(
            id,
            kind,
            kind_id,
            source_entity_id,
            target_entity_id,
            source_entity_def_id,
            target_entity_def_id,
            source_entities_id_name,
            target_entities_id_name,
            text_attrs,
            smallint_attrs,
            int_attrs,
            boolean_attrs,
        )
        .await;
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_ent_link(id(), format!("{} ({})", kind(), id())) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-4",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity Link"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkListPage {},
                                "X"
                            }
                        }
                        EntityLinkForm {
                            source_entity_id,
                            source_entities_id_name,
                            target_entity_id,
                            target_entities_id_name,
                            text_attrs,
                            smallint_attrs,
                            int_attrs,
                            boolean_attrs,
                            action,
                        }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-300 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_delete_confirm.set(true);
                                },
                                "Delete"
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 min-w-[90px] disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: update_btn_disabled,
                                onclick: move |_| {
                                    async move {
                                        match action().clone() {
                                            Action::View => {
                                                action.set(Action::Edit);
                                            }
                                            Action::Delete => {
                                                navigator().push(Route::EntityListPage {});
                                            }
                                            Action::Edit => {
                                                if action_done() {
                                                    navigator().push(Route::EntityListPage {});
                                                } else {
                                                    handle_update(
                                                            id(),
                                                            source_entity_id(),
                                                            target_entity_id(),
                                                            text_attrs().values().cloned().collect(),
                                                            smallint_attrs().values().cloned().collect(),
                                                            int_attrs().values().cloned().collect(),
                                                            boolean_attrs().values().cloned().collect(),
                                                            action,
                                                            action_done,
                                                            err,
                                                        )
                                                        .await;
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                if action() == Action::View || (action() == Action::Edit && action_done()) {
                                    "Edit"
                                } else if action() == Action::Delete {
                                    "Close"
                                } else {
                                    "Update"
                                }
                            }
                        }
                    }
                }
            }
            if show_delete_confirm() {
                if action() != Action::Delete {
                    ConfirmationModal {
                        title: "Confirm Delete",
                        content: "Are you sure you want to delete this entity link?",
                        action,
                        show_modal: show_delete_confirm,
                        action_handler: move |_| {
                            spawn(async move {
                                log::debug!("Calling handle_delete ...");
                                handle_delete(&id(), action_done, err).await;
                            });
                        },
                    }
                }
            } else if action_done() {
                if err().is_none() {
                    AcknowledgeModal {
                        title: "Confirmation",
                        content: if action() == Action::Delete { vec!["The entity link has been successfully deleted.".into()] } else { vec!["The entity link has been successfully updated.".into()] },
                        action_handler: move |_| {
                            navigator().push(Route::EntityLinkListPage {});
                        },
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: vec!["Failed to update the entity link. Reason:".into(), err.unwrap()],
                        action_handler: move |_| {
                            action_done.set(false);
                            err.set(None);
                        },
                    }
                }
            }
        }
    }
}

async fn init(
    id: Signal<Id>,
    mut kind: Signal<Name>,
    mut kind_id: Signal<Id>,
    mut source_entity_id: Signal<Id>,
    mut target_entity_id: Signal<Id>,
    mut source_entity_def_id: Signal<Id>,
    mut target_entity_def_id: Signal<Id>,
    mut source_entities_id_name: Signal<IndexMap<Id, Name>>,
    mut target_entities_id_name: Signal<IndexMap<Id, Name>>,
    mut text_attrs: Signal<IndexMap<Id, TextAttribute>>,
    mut smallint_attrs: Signal<IndexMap<Id, SmallintAttribute>>,
    mut int_attrs: Signal<IndexMap<Id, IntegerAttribute>>,
    mut boolean_attrs: Signal<IndexMap<Id, BooleanAttribute>>,
) {
    match get_entity_link(id()).await {
        Ok(Some(ent_link)) => {
            log::debug!("[EntityLinkPage] Based on id '{id}', got {:?}", ent_link);

            kind.set(ent_link.kind);
            kind_id.set(ent_link.def_id);
            source_entity_id.set(ent_link.source_entity_id);
            target_entity_id.set(ent_link.target_entity_id);

            match get_entity_link_def(kind_id()).await {
                Ok(eld_opt) => {
                    if let Some(eld) = eld_opt {
                        source_entity_def_id.set(eld.source_entity_def_id);
                        target_entity_def_id.set(eld.target_entity_def_id);
                    }
                }
                Err(e) => {
                    log::error!(
                        "[EntityLinkPage] Failed to get entity link def w/ id: '{}'. Reason: '{}'.",
                        kind_id(),
                        e
                    );
                }
            }

            let attrs: IndexMap<Id, TextAttribute> = ent_link
                .text_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            text_attrs.set(attrs);
            let attrs: IndexMap<Id, SmallintAttribute> = ent_link
                .smallint_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            smallint_attrs.set(attrs);
            let attrs: IndexMap<Id, IntegerAttribute> = ent_link
                .int_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            int_attrs.set(attrs);
            let attrs: IndexMap<Id, BooleanAttribute> = ent_link
                .boolean_attributes
                .iter()
                .map(|attr| (attr.name.clone().into(), attr.clone()))
                .collect();
            boolean_attrs.set(attrs);
        }
        Ok(None) => {
            log::error!("[EntityLinkPage] Entity link with id '{id}' not found.");
            return;
        }
        Err(err) => {
            log::error!("[EntityLinkPage] Failed to get entity by id '{id}'. Cause: {err}");
            return;
        }
    }
    match list_entities_by_def_id(source_entity_def_id()).await {
        Ok(source_entities) => {
            let mut id_name_map = IndexMap::new();
            for ent in source_entities {
                id_name_map.insert(ent.id, ent.listing_attr_value);
            }
            source_entities_id_name.set(id_name_map);
        }
        Err(e) => {
            log::error!("[EntityLinkNewPage] Error loading source entities by def id:'{}': {}", kind_id(), e);
        }
    }
    match list_entities_by_def_id(target_entity_def_id()).await {
        Ok(target_entities) => {
            let mut id_name_map = IndexMap::new();
            for ent in target_entities {
                id_name_map.insert(ent.id, ent.listing_attr_value);
            }
            target_entities_id_name.set(id_name_map);
        }
        Err(e) => {
            log::error!("[EntityLinkNewPage] Error loading target entities by def id:'{}': {}", kind_id(), e);
        }
    }

    log::debug!(
        "[EntityLinkPage] Loaded source_entities_id_name: {:?} target_entities_id_name: {:?}",
        source_entities_id_name(),
        target_entities_id_name()
    );
}

async fn handle_update(
    ent_link_id: Id,
    source_entity_id: Id,
    target_entity_id: Id,
    text_attributes: Vec<TextAttribute>,
    smallint_attributes: Vec<SmallintAttribute>,
    int_attributes: Vec<IntegerAttribute>,
    boolean_attributes: Vec<BooleanAttribute>,
    mut action: Signal<Action>,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    if (source_entity_id == Id::default()) || (target_entity_id == Id::default()) {
        action_done.set(true);
        action.set(Action::Edit);
        err.set(Some("Both source and target entities must be selected.".to_string()));
        return;
    }

    let item = EntityLink {
        id: ent_link_id.clone(),
        kind: Name::default(), // Not used further.
        def_id: Id::default(), // Not used further.
        source_entity_id,
        target_entity_id,
        text_attributes,
        smallint_attributes,
        int_attributes,
        boolean_attributes,
    };

    log::debug!("Updating entity link '{:?}' ... ", item);

    match update_entity_link(item).await {
        Ok(_) => {
            action_done.set(true);
            action.set(Action::View);
            err.set(None);
        }
        Err(e) => {
            action_done.set(true);
            action.set(Action::Edit);
            err.set(Some(e.to_string()));
        }
    }
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!("[EntityLinkPage] Deleting entity link w/ id {:?}", id);
    match remove_entity_link(id.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
        }
        Err(e) => {
            action_done.set(true);
            err.set(Some(e.to_string()));
        }
    }
}
