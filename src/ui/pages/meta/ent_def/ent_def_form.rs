use crate::domain::model::Id;
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Props, PartialEq, Clone)]
pub struct EntityDefFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub ordered_included_attr_defs: Signal<IndexMap<Id, (String, Option<String>)>>,
    pub ordered_included_attrs_order_change: Signal<(usize, usize)>,
    pub ordered_included_attrs_dragging_in_progress: Signal<bool>,
    pub listing_attr_def_id: Signal<Id>,
    pub all_attr_defs: Signal<IndexMap<Id, (String, Option<String>)>>,
    pub action: String,
    pub action_done: Signal<bool>,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityDefForm(props: EntityDefFormProps) -> Element {
    //
    let EntityDefFormProps {
        mut name,
        mut description,
        mut ordered_included_attr_defs,
        mut ordered_included_attrs_order_change,
        mut ordered_included_attrs_dragging_in_progress,
        mut listing_attr_def_id,
        mut all_attr_defs,
        action,
        action_done,
        mut err,
    } = props;

    let is_view = action == "View";

    let mut selected_attr_def_id = use_signal(|| Id::default());
    let mut selected_attr_def_name = use_signal(|| "".to_string());
    let mut selected_attr_def_desc = use_signal(|| None);

    let mut drag_source_attr_index = use_signal(|| 0usize);
    let mut drag_target_attr_index = use_signal(|| 0usize);

    use_effect(move || {
        let attr_source_index = drag_source_attr_index();
        let attr_target_index = drag_target_attr_index();
        ordered_included_attrs_order_change.set((attr_source_index, attr_target_index));
    });

    rsx! {
        div { class: "mt-4 space-y-4",
            // "Name" section.
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Name" }
                input {
                    key: "name_{action}",
                    class: "px-3 py-1 min-w-80",
                    r#type: "text",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view,
                    autofocus: !is_view,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        if !is_view {
                            _ = evt.set_focus(true).await;
                        }
                    },
                }
            }
            // "Description" section.
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Description" }
                textarea {
                    class: "px-3 py-2 min-w-80",
                    rows: 3,
                    cols: 32,
                    value: "{description}",
                    readonly: is_view,
                    maxlength: 256,
                    oninput: move |evt| {
                        description.set(evt.value());
                    },
                }
            }
            // "Attributes" section.
            div { class: "flex",
                p { class: "min-w-32 text-gray-500", "Attributes" }
                div {
                    for (index , (id , (name , desc))) in ordered_included_attr_defs().into_iter().enumerate() {
                        div { class: if is_view { "flex justify-between min-w-80" } else { "flex justify-between min-w-80 cursor-row-resize" },
                            p {
                                class: "pl-3 pr-3",
                                draggable: if is_view { false } else { true },
                                ondragstart: move |_| {
                                    drag_source_attr_index.set(index);
                                    ordered_included_attrs_dragging_in_progress.set(true);
                                },
                                ondragover: move |_| {
                                    if index != drag_target_attr_index() {
                                        drag_target_attr_index.set(index);
                                    }
                                },
                                ondragend: move |_| {
                                    ordered_included_attrs_dragging_in_progress.set(false);
                                },
                                if desc.is_some() {
                                    "{name}   ({desc.clone().unwrap()})"
                                } else {
                                    "{name}"
                                }
                            }
                            button {
                                class: "text-red-200 hover:text-red-500 hover:bg-red-100 disabled:text-white disabled:hover:bg-white ml-4 px-3 py-0 rounded-xl transition duration-200",
                                display: if is_view { "none" } else { "inline" },
                                // Remove the item from `ordered_included_attr_defs` and put it back into `all_attr_defs`.
                                onclick: move |_| {
                                    let id = id.clone();
                                    let name = name.clone();
                                    let desc = desc.clone();
                                    let mut temp = ordered_included_attr_defs();
                                    temp.swap_remove(&id);
                                    ordered_included_attr_defs.set(temp);
                                    let mut temp = all_attr_defs();
                                    temp.insert(id.clone(), (name, desc));
                                    all_attr_defs.set(temp);
                                    if listing_attr_def_id() == id {
                                        listing_attr_def_id.set(Id::default());
                                    }
                                },
                                "-"
                            }
                        }
                    }
                }
            }
            // "Listing attribute" section.
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Listing attribute" }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        listing_attr_def_id.set(evt.value().into());
                        log::debug!("[EntityDefForm] selected_attr_def_id: {:?}", evt.value());
                    },
                    for (id , (name , desc)) in ordered_included_attr_defs() {
                        option {
                            value: "{id}",
                            selected: "{listing_attr_def_id() == id}",
                            if desc.is_some() {
                                "{name}   ({desc.as_ref().unwrap()})"
                            } else {
                                "{name}"
                            }
                        }
                    }
                }
            }
            hr { class: "mt-8 mb-1" }
            // "Select an attribute definition to include" section.
            div {
                class: "flex",
                display: if action == "View" || action == "Delete" || (action == "Edit" && action_done()) { "none" } else { "block" },
                label { class: "pr-3 py-1 min-w-28", "" }
                p { class: "text-gray-500 font-sm", "Select an attribute definition to include." }
                select {
                    class: "px-3 py-2 min-w-80",
                    multiple: false,
                    disabled: is_view,
                    oninput: move |evt| {
                        selected_attr_def_id.set(evt.value().into());
                        let attr_def = all_attr_defs().get(&selected_attr_def_id()).unwrap().clone();
                        selected_attr_def_name.set(attr_def.0);
                        selected_attr_def_desc.set(attr_def.1);
                    },
                    option { value: "", selected: true, "" }
                    for (id , (name , desc)) in all_attr_defs() {
                        option {
                            value: "{id}",
                            selected: "{selected_attr_def_id() == id}",
                            if desc.is_some() {
                                "{name}   ({desc.as_ref().unwrap()})"
                            } else {
                                "{name}"
                            }
                        }
                    }
                }
                button {
                    class: "bg-slate-100 text-slate-600 hover:text-gray-800 ml-4 px-3 rounded-lg transition duration-200",
                    disabled: is_view,
                    onclick: move |_| {
                        if selected_attr_def_id().is_empty() {
                            return;
                        }
                        if listing_attr_def_id().is_empty() {
                            listing_attr_def_id.set(selected_attr_def_id());
                        }
                        let mut included = ordered_included_attr_defs();
                        included
                            .insert(
                                selected_attr_def_id(),
                                (selected_attr_def_name(), selected_attr_def_desc()),
                            );
                        ordered_included_attr_defs.set(included);
                        let mut attr_defs = all_attr_defs();
                        attr_defs.swap_remove(&selected_attr_def_id());
                        all_attr_defs.set(attr_defs);
                        selected_attr_def_id.set(Id::default());
                        selected_attr_def_name.set("".to_string());
                        err.set(None);
                    },
                    "+"
                }
            }
        }
    }
}
