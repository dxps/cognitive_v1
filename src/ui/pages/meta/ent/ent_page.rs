use crate::{
    domain::model::{AttributeValueType, BooleanAttribute, Entity, Id, IntegerAttribute, SmallintAttribute, TextAttribute},
    server::fns::{get_entity, remove_entity, update_entity},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, EntityForm, Nav},
        routes::Route,
        Action,
    },
};
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Props, Clone)]
pub struct EntityPageProps {
    id: Id,
}

#[component]
pub fn EntityPage(props: EntityPageProps) -> Element {
    //
    let id = use_signal(|| props.id);
    let def_id = use_signal(|| Id::default());
    let kind = use_signal(|| "".to_string());
    let listing_attr_def_id = use_signal(|| Id::default());

    let text_attrs = use_signal::<HashMap<Id, TextAttribute>>(|| HashMap::new());
    let smallint_attrs = use_signal::<HashMap<Id, SmallintAttribute>>(|| HashMap::new());
    let int_attrs = use_signal::<HashMap<Id, IntegerAttribute>>(|| HashMap::new());
    let boolean_attrs = use_signal::<HashMap<Id, BooleanAttribute>>(|| HashMap::new());
    let attributes_order = use_signal::<Vec<(AttributeValueType, Id)>>(|| Vec::new());

    let mut show_delete_confirm = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let action_done = use_signal(|| false);
    let err: Signal<Option<String>> = use_signal(|| None);

    use_future(move || async move {
        init(
            id,
            kind,
            def_id,
            attributes_order,
            text_attrs,
            smallint_attrs,
            int_attrs,
            boolean_attrs,
            listing_attr_def_id,
        )
        .await;
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb {
                paths: Route::get_path_to_ent(
                    Route::EntityPage { id: id() },
                    format!("{} ({})", kind(), id()),
                ),
            }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-10",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Entity"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityListPage {},
                                "X"
                            }
                        }
                        EntityForm {
                            attributes_order,
                            text_attrs,
                            smallint_attrs,
                            int_attrs,
                            boolean_attrs,
                            action: action(),
                        }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-300 hover:text-red-600 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_delete_confirm.set(true);
                                },
                                "Delete"
                            }
                            // Show the buttons's action result in the UI.
                            div { class: "min-w-[400px] max-w-[400px] text-sm flex justify-center items-center",
                                if err().is_some() {
                                    span { class: "text-red-600", {err().unwrap()} }
                                } else if action_done() {
                                    span { class: "text-green-600",
                                        {if action() == Action::Edit { "Successfully updated" } else { "" }}
                                    }
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 min-w-[90px] disabled:text-gray-300 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
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
                                                            kind(),
                                                            def_id(),
                                                            text_attrs(),
                                                            smallint_attrs(),
                                                            int_attrs(),
                                                            boolean_attrs(),
                                                            listing_attr_def_id(),
                                                            action_done,
                                                            err,
                                                        )
                                                        .await;
                                                    action.set(Action::View);
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
                        content: "Are you sure you want to delete this entity?",
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
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete { vec!["The entity has been successfully deleted.".into()] } else { vec!["The entity has been successfully updated.".into()] },
                    action_handler: move |_| {
                        navigator().push(Route::EntityListPage {});
                    },
                }
            }
        }
    }
}

async fn init(
    id: Signal<Id>,
    mut kind: Signal<String>,
    mut def_id: Signal<Id>,
    mut attributes_order: Signal<Vec<(AttributeValueType, Id)>>,
    mut text_attrs: Signal<HashMap<Id, TextAttribute>>,
    mut smallint_attrs: Signal<HashMap<Id, SmallintAttribute>>,
    mut int_attrs: Signal<HashMap<Id, IntegerAttribute>>,
    mut boolean_attrs: Signal<HashMap<Id, BooleanAttribute>>,
    mut listing_attr_def_id: Signal<Id>,
) {
    match get_entity(id()).await {
        Ok(Some(ent)) => {
            log::debug!("[EntityPage] Based on id {id}, got entity {:?}", ent);
            attributes_order.set(ent.attributes_order);

            let mut ent_text_attrs = HashMap::new();
            ent.text_attributes.iter().for_each(|attr| {
                ent_text_attrs.insert(attr.id.clone(), attr.clone());
            });
            text_attrs.set(ent_text_attrs);

            let mut ent_smallint_attrs = HashMap::new();
            ent.smallint_attributes.iter().for_each(|attr| {
                ent_smallint_attrs.insert(attr.id.clone(), attr.clone());
            });
            smallint_attrs.set(ent_smallint_attrs);

            let mut ent_int_attrs = HashMap::new();
            ent.int_attributes.iter().for_each(|attr| {
                ent_int_attrs.insert(attr.id.clone(), attr.clone());
            });
            int_attrs.set(ent_int_attrs);

            let mut ent_boolean_attrs = HashMap::new();
            ent.boolean_attributes.iter().for_each(|attr| {
                ent_boolean_attrs.insert(attr.id.clone(), attr.clone());
            });
            boolean_attrs.set(ent_boolean_attrs);

            kind.set(ent.kind);
            def_id.set(ent.def_id);
            listing_attr_def_id.set(ent.listing_attr_def_id);
        }
        Ok(None) => {
            log::error!("[EntityPage] Entity with id '{id}' not found");
        }
        Err(err) => {
            log::error!("[EntityPage] Failed to get entity by id '{id}'. Cause: {err}");
        }
    }
}

async fn handle_update(
    ent_id: Id,
    kind: String,
    def_id: Id,
    text_attributes: HashMap<Id, TextAttribute>,
    smallint_attributes: HashMap<Id, SmallintAttribute>,
    int_attributes: HashMap<Id, IntegerAttribute>,
    boolean_attributes: HashMap<Id, BooleanAttribute>,
    listing_attr_def_id: Id,
    mut saved: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    //
    let ent = Entity::new_with_id_attrs(
        ent_id,
        kind,
        def_id,
        text_attributes.values().cloned().collect(),
        smallint_attributes.values().cloned().collect(),
        int_attributes.values().cloned().collect(),
        boolean_attributes.values().cloned().collect(),
        listing_attr_def_id,
    );

    log::debug!("Updating entity '{:?}' ... ", ent);

    match update_entity(ent).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}

async fn handle_delete(id: &Id, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting entity w/ id {:?}", id);
    match remove_entity(id.clone()).await {
        Ok(_) => {
            action_done.set(true);
            err.set(None);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
