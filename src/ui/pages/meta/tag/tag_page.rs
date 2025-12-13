use crate::{
    domain::model::{Id, Tag},
    server::fns::{remove_tag, update_tag},
    ui::{
        comps::{AcknowledgeModal, Breadcrumb, ConfirmationModal, Nav, TagForm},
        routes::Route,
        Action, UI_STATE,
    },
};
use dioxus::prelude::*;

#[component]
pub fn TagPage(id: Id) -> Element {
    //
    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());

    let mut show_delete_confirm = use_signal(|| false);
    let mut action = use_signal(|| Action::View);
    let action_done = use_signal(|| false);
    let mut err: Signal<Option<String>> = use_signal(|| None);

    let tid = id.clone();
    let did = id.clone();
    use_future(move || {
        let id = tid.clone();
        async move {
            if let Some(t) = UI_STATE.get_tag(&id).await {
                name.set(t.name.clone());
                description.set(t.description.unwrap_or_default());
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path_to_tag(Route::TagPage { id: id.clone() }, name()) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "{action} Tag"
                            }
                            Link {
                                class: "text-gray-500 hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagListPage {},
                                "X"
                            }
                        }
                        TagForm { name, description, action: action() }
                        div { class: "flex justify-between mt-8",
                            button {
                                class: "text-red-400 bg-slate-50 hover:text-red-700 hover:bg-red-100 drop-shadow-sm px-4 rounded-md",
                                onclick: move |_| {
                                    show_delete_confirm.set(true);
                                },
                                "Delete"
                            }
                            // Show the buttons' action result in the UI.
                            div { class: "min-w-[350px] max-w-[350px] mt-1 pl-2",
                                if err().is_some() {
                                    span { class: "text-red-600 flex justify-center",
                                        { err().unwrap() }
                                    }
                                } else if action_done() {
                                    span { class: "text-green-600 flex justify-center",
                                        {
                                            if action() == Action::Edit {
                                                "Successfully updated"
                                            } else {
                                                ""
                                            }
                                        }
                                    }
                                }
                            }
                            button {
                                class: "bg-gray-100 enabled:hover:bg-green-100 disabled:text-gray-400 hover:disabled:bg-gray-100 drop-shadow-sm px-4 rounded-md",
                                disabled: action() == Action::Delete,
                                onclick: move |_| {
                                    let id = id.clone();
                                    let description = match description().is_empty() {
                                        true => None,
                                        false => Some(description()),
                                    };
                                    let curr_action = action().clone();
                                    async move {
                                        if curr_action == Action::View {
                                            action.set(Action::Edit);
                                        } else {
                                            if name().is_empty() {
                                                err.set(Some("Name cannot be empty".to_string()));
                                                return;
                                            }
                                            let tag = Tag::new(id, name(), description);
                                            handle_update(tag, action_done, err).await;
                                            action.set(Action::View);
                                        }
                                    }
                                },
                                if action() == Action::View || action_done() {
                                    "Edit"
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
                        content: "Are you sure you want to delete this tag?",
                        action,
                        show_modal: show_delete_confirm,
                        action_handler: move |_| {
                            let id = did.clone();
                            action.set(Action::Delete);
                            spawn(async move {
                                log::debug!("Calling handle_delete ...");
                                handle_delete(id, action_done, err).await;
                            });
                        }
                    }
                }
            }
            if action_done() {
                AcknowledgeModal {
                    title: "Confirmation",
                    content: if action() == Action::Delete {
                        vec!["The tag has been successfully deleted.".into()]
                    } else {
                        vec!["The tag has been successfully updated.".into()]
                    },
                    action_handler: move |_| {
                        navigator().push(Route::TagListPage {});
                    }
                }
            }
        }
    }
}

async fn handle_update(tag: Tag, mut action_complete: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Updating tag: {:?}", tag);
    match update_tag(tag.clone()).await {
        Ok(_) => {
            action_complete.set(true);
            err.set(None);
            UI_STATE.update_tag(tag).await;
        }
        Err(e) => {
            action_complete.set(false);
            err.set(Some(e.to_string()));
        }
    }
}

async fn handle_delete(id: Id, mut saved: Signal<bool>, mut err: Signal<Option<String>>) {
    //
    log::debug!(">>> Deleting tag: {:?}", id);
    match remove_tag(id.clone()).await {
        Ok(_) => {
            saved.set(true);
            err.set(None);
            UI_STATE.remove_tag(id).await;
        }
        Err(e) => {
            saved.set(false);
            err.set(Some(e.to_string()));
        }
    }
}
