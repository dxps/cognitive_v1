use crate::{
    domain::model::{EntityDef, Id},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, Nav},
        pages::{meta::ent_def::fetch_all_attr_defs, EntityDefForm},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;
use indexmap::IndexMap;
use log::debug;

pub fn EntityDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    let mut ordered_included_attr_defs = use_signal::<IndexMap<Id, (String, Option<String>)>>(|| IndexMap::new());
    let mut ordered_included_attrs_order_change = use_signal::<(usize, usize)>(|| (0, 0));
    let ordered_included_attrs_dragging_in_progress = use_signal(|| false);
    let mut included_attr_defs = ordered_included_attr_defs();

    let listing_attr_def_id = use_signal(|| Id::default());

    let mut all_attr_defs = use_signal(|| IndexMap::<Id, (String, Option<String>)>::new());

    let create_btn_disabled = use_memo(move || name().is_empty() || ordered_included_attr_defs().is_empty());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let action_done = use_signal(|| false);

    use_future(move || async move {
        all_attr_defs.set(fetch_all_attr_defs().await);
    });

    // React to DnD changes.
    use_effect(move || {
        let (source_attr_index, target_attr_index) = ordered_included_attrs_order_change();
        if !ordered_included_attrs_dragging_in_progress() && source_attr_index != target_attr_index {
            let mut changed_attr_defs = included_attr_defs.clone();
            let range: &mut dyn Iterator<Item = usize> = if source_attr_index < target_attr_index {
                &mut (source_attr_index..target_attr_index)
            } else {
                &mut (target_attr_index..source_attr_index).rev().into_iter()
            };
            for index in range {
                changed_attr_defs.swap_indices(index, index + 1);
            }
            included_attr_defs = changed_attr_defs.clone();
            ordered_included_attr_defs.set(changed_attr_defs);
            ordered_included_attrs_order_change.set((0, 0));
            debug!(
                ">>> [EntityDefNewPage] After DnD change, ordered_included_attr_defs: {:?}",
                ordered_included_attr_defs(),
            );
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-10",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create Entity Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefListPage {},
                                "X"
                            }
                        }
                        EntityDefForm {
                            name,
                            description,
                            ordered_included_attr_defs,
                            ordered_included_attrs_order_change,
                            ordered_included_attrs_dragging_in_progress,
                            listing_attr_def_id,
                            all_attr_defs,
                            action: Action::Edit,
                            action_done,
                            err,
                        }
                        div { class: "flex justify-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: create_btn_disabled(),
                                onclick: move |_| {
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::EntityDefListPage {});
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            if ordered_included_attr_defs().is_empty() {
                                                err.set(Some("Include at least one attribute".to_string()));
                                                return;
                                            }
                                            handle_create_ent_def(
                                                    name(),
                                                    description.clone(),
                                                    listing_attr_def_id(),
                                                    ordered_included_attr_defs(),
                                                    all_attr_defs(),
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
                        content: vec!["The entity definition has been successfully created.".into()],
                        action_handler: move |_| {
                            navigator().push(Route::EntityDefListPage {});
                        },
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: vec!["Failed to create the entity definition. Reason:".into(), err.unwrap()],
                        action_handler: move |_| {
                            navigator().push(Route::EntityDefListPage {});
                        },
                    }
                }
            }
        }
    }
}

async fn handle_create_ent_def(
    name: String,
    description: Option<String>,
    listing_attr_def_id: Id,
    included_attr_defs: IndexMap<Id, (String, Option<String>)>,
    all_attr_defs: IndexMap<Id, (String, Option<String>)>,
    mut action_done: Signal<bool>,
    mut err: Signal<Option<String>>,
) {
    log::debug!(
        "[handle_create_ent_def] Creating ent def w/ included_attr_defs: {:?} and all_attr_defs: {:?} ",
        included_attr_defs,
        all_attr_defs
    );

    let included_attr_defs: IndexMap<Id, String> = included_attr_defs
        .iter()
        .map(|(id, (name, _))| (id.clone(), name.clone()))
        .collect();
    let mut ent_def = EntityDef::new_with_attr_def_ids("".into(), name, description, included_attr_defs, listing_attr_def_id);
    log::debug!("[handle_create_ent_def] Creating ent def: {:?}: ", ent_def);
    match crate::server::fns::create_entity_def(ent_def.clone()).await {
        Ok(id) => {
            action_done.set(true);
            err.set(None);
            ent_def.id = id;
            UI_STATE.add_ent_def(ent_def);
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
