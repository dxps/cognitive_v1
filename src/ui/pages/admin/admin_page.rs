use dioxus::prelude::*;

use crate::ui::{
    comps::{Breadcrumb, Nav},
    pages::LoginIsRequiredPage,
    routes::Route,
    UiStorage, UI_STATE,
};

#[component]
pub fn AdminPage() -> Element {
    //
    if *UI_STATE.app_ready.read() == false {
        return rsx! { "Loading..." };
    }
    let state = use_context::<Signal<UiStorage>>();
    if state().current_user.is_none() {
        rsx! {
            LoginIsRequiredPage {}
        }
    } else {
        render_page()
    }
}

fn render_page() -> Element {
    //
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AdminPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        h5 { class: "mb-2 block text-lg font-semibold leading snug tracking-normal text-gray-500 antialiased",
                            "Model Management"
                        }
                        p { class: "block font-sans text-base text-gray-500 leading-relaxed antialiased",
                            "Manage the definitions and instances of attributes, entities, entities links, and tags."
                        }
                        hr { class: "mt-2 mb-4" }
                        div { class: "flex",
                            div { class: "pr-3 flex flex-col grow mr-1",
                                h6 { class: "px-4 mb-2 pt-2 pb-1 block font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                    "Definitions"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::EntityDefListPage {},
                                    "Entities"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::EntityLinkDefListPage {},
                                    "Entity Links"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::AttributeDefListPage {},
                                    "Attributes"
                                }
                            }
                            div { class: "pr-3 flex flex-col grow ml-1",
                                h6 { class: "px-4 mb-2 pt-2 pb-1 block font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                    "Instances"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::EntityListPage {},
                                    "Entities"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::EntityLinkListPage {},
                                    "Entity Links"
                                }
                                Link {
                                    class: "py-2 px-4 rounded-lg transition duration-200",
                                    to: Route::TagListPage {},
                                    "Tags"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
