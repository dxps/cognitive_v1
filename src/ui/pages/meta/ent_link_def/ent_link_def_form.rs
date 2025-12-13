use crate::{
    domain::model::{Cardinality, Id},
    ui::{comps::Select, pages::Name},
};
use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Clone, Debug, Props, PartialEq)]
pub struct EntityLinkDefFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub cardinality_id: Signal<Id>,
    pub source_ent_def_id: Signal<Id>,
    pub target_ent_def_id: Signal<Id>,
    pub ent_defs: Signal<IndexMap<Id, Name>>,
    pub included_attr_defs: Signal<IndexMap<Id, (Name, Option<String>)>>,
    pub all_attr_defs: Signal<IndexMap<Id, (Name, Option<String>)>>,
    pub action: String,
    pub action_done: Signal<bool>,
    pub err: Signal<Option<String>>,
}

#[component]
pub fn EntityLinkDefForm(props: EntityLinkDefFormProps) -> Element {
    //
    log::debug!("[EntityLinkDefForm] props: {:?}", props);

    let EntityLinkDefFormProps {
        mut name,
        mut description,
        cardinality_id,
        source_ent_def_id,
        target_ent_def_id,
        ent_defs,
        mut included_attr_defs,
        mut all_attr_defs,
        action,
        action_done,
        mut err,
    } = props;

    let is_view = action == "View";

    let cardinality_options = use_signal(|| Cardinality::get_select_variants());
    let mut selected_attr_def_id = use_signal(|| Id::default());
    let mut selected_attr_def_name = use_signal(|| "".to_string());
    let mut selected_attr_def_desc = use_signal(|| None);

    rsx! {
        div { class: "mt-4 space-y-4",
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
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Source" }
                Select {
                    items: ent_defs,
                    selected_item_id: source_ent_def_id,
                    disabled: is_view,
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Target" }
                Select {
                    items: ent_defs,
                    selected_item_id: target_ent_def_id,
                    disabled: is_view,
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-32 text-gray-500", "Cardinality" }
                Select {
                    items: cardinality_options,
                    selected_item_id: cardinality_id,
                    default_selected_item_id: Some(cardinality_id()),
                    disabled: is_view,
                }
            }
            div { class: "flex mb-12",
                p { class: "min-w-32 text-gray-500", "Attributes" }
                div {
                    for (id , (name , desc)) in included_attr_defs() {
                        div { class: "flex justify-between min-w-80",
                            p { class: "pl-3 pr-3", "{name}" }
                            button {
                                class: "text-red-200 hover:text-red-500 hover:bg-red-100 disabled:text-white disabled:hover:bg-white ml-4 px-3 py-0 rounded-xl transition duration-200",
                                disabled: is_view,
                                // Remove the item from `included_attr_defs` and put it back into `all_attr_defs`.
                                onclick: move |_| {
                                    let id = id.clone();
                                    let name = name.clone();
                                    let desc = desc.clone();
                                    let mut temp = included_attr_defs();
                                    temp.swap_remove(&id);
                                    included_attr_defs.set(temp);
                                    let mut temp = all_attr_defs();
                                    temp.insert(id.clone(), (name, desc));
                                    all_attr_defs.set(temp);
                                },
                                "-"
                            }
                        }
                    }
                }
            }
            div {
                class: "flex",
                display: if action == "View" || action == "Delete" || (action == "Edit" && action_done()) { "none" } else { "block" },
                label { class: "pr-3 py-1 min-w-28", "" }
                p { class: "text-gray-500 font-sm",
                    "Optionally, add one or more attribute definitions to it."
                }
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
                        let mut included = included_attr_defs();
                        included
                            .insert(
                                selected_attr_def_id(),
                                (selected_attr_def_name(), selected_attr_def_desc()),
                            );
                        included_attr_defs.set(included);
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
