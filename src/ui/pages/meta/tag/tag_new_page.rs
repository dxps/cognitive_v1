use crate::{
    domain::model::Tag,
    server::fns::create_tag,
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, Nav, TagForm},
        routes::Route,
        Action, UI_STATE,
    },
};

use dioxus::prelude::*;

#[component]
pub fn TagNewPage() -> Element {
    //
    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());

    let mut err: Signal<Option<String>> = use_signal(|| None);
    let action_done = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::TagNewPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-10",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Create a Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "X"
                            }
                        }
                        TagForm { name, description, action: Action::Edit }
                        div { class: "flex justify-betweent mt-8",
                            // Show the button's action result in the UI.
                            div { class: "min-w-[440px] max-w-[440px]",
                                if err().is_some() {
                                    span { class: "text-red-600 flex justify-center",
                                        { err().unwrap() }
                                    }
                                } else if action_done() {
                                    ""
                                }
                            }
                            button {
                                class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        if action_done() {
                                            navigator().push(Route::TagListPage {});
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            let description = match description().is_empty() {
                                                true => None,
                                                false => Some(description()),
                                            };
                                            handle_create_tag(name(), description.clone(), action_done, err).await;
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
                AcknowledgeModal {
                    title: "Confirmation",
                    content: vec!["The tag has been successfully created.".into()],
                    action_handler: move |_| {
                        navigator().push(Route::TagListPage {});
                    }
                }
            }
        }
    }
}

async fn handle_create_tag(name: String, description: Option<String>, mut action_done: Signal<bool>, mut err: Signal<Option<String>>) {
    match create_tag(name.clone(), description.clone()).await {
        Ok(id) => {
            action_done.set(true);
            err.set(None);
            UI_STATE.add_tag(Tag::new(id.clone(), name, description)).await;
        }
        Err(e) => {
            action_done.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
