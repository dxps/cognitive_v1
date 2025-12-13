use crate::{
    domain::model::{AttributeDef, Id},
    server::fns::create_attribute_def,
    ui::{
        comps::{AcknowledgeModal, AttributeDefForm, Breadcrumb, Nav},
        routes::Route,
        Action, UI_STATE,
    },
};

use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn AttributeDefNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let value_type = use_signal(|| "text".to_string());
    let default_value = use_signal(|| "".to_string());
    let is_required = use_signal(|| false);
    let tag_id = use_signal(|| Id::default());
    let mut tags = use_signal(|| Arc::new(Vec::new()));

    let create_btn_disabled = use_memo(move || name().is_empty());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut action_done = use_signal(|| false);

    use_future(move || async move {
        tags.set(UI_STATE.get_tags_list().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-10",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create an Attribute Definition"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefListPage {},
                                "X"
                            }
                        }
                        AttributeDefForm {
                            name,
                            description,
                            value_type,
                            default_value,
                            is_required,
                            tag_id,
                            tags: tags(),
                            action: Action::Create
                        }
                        div { class: "grid justify-items-end mt-8",
                            button {
                                class: "bg-gray-100 hover:bg-green-100 disabled:text-gray-400 disabled:hover:bg-gray-200 drop-shadow-sm px-4 py-2 rounded-md",
                                disabled: create_btn_disabled(),
                                onclick: move |_| {
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::AttributeDefListPage {});
                                        } else {
                                            let description = match description().is_empty() {
                                                true => None,
                                                false => Some(description()),
                                            };
                                            let tag_id = match tag_id().is_empty() {
                                                true => None,
                                                false => Some(tag_id()),
                                            };
                                            let item = AttributeDef {
                                                id: Id::default(),
                                                name: name(),
                                                description,
                                                value_type: value_type().into(),
                                                default_value: default_value(),
                                                is_required: is_required(),
                                                tag_id,
                                            };
                                            create_handler(item, action_done, err).await;
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
                        content: vec!["The attribute definition has been successfully created.".into()],
                        action_handler: move |_| {
                            navigator().push(Route::AttributeDefListPage {});
                        }
                    }
                } else {
                    AcknowledgeModal {
                        title: "Error",
                        content: vec!["Failed to create the attribute definition. Reason:".into(), err.unwrap()],
                        action_handler: move |_| {
                            action_done.set(false);
                            err.set(None);
                        }
                    }
                }
            }
        }
    }
}

async fn create_handler(item: AttributeDef, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    log::debug!("Creating an attribute definition {:?}: ", item);
    err.set(match create_attribute_def(item).await {
        Ok(_) => None,
        Err(e) => {
            if let ServerFnError::ServerError(s) = e {
                Some(s)
            } else {
                Some(e.to_string())
            }
        }
    });
    action_done.set(true);
}
