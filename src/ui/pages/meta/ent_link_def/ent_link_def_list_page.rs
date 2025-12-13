use dioxus::prelude::*;

use crate::{
    domain::model::EntityLinkDef,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};

#[component]
pub fn EntityLinkDefListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<EntityLinkDef>>(|| vec![]);

    use_future(move || async move {
        entries.set(UI_STATE.get_ent_link_def_list().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entity Link Definitions"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkDefNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for item in entries() {
                            EntityLinkDefCard { item: item.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityLinkDefCard(item: EntityLinkDef) -> Element {
    //
    let description = match item.description {
        Some(description) => description,
        None => String::from(""),
    };
    rsx! {
        Link {
            to: Route::EntityLinkDefPage {
                id: item.id,
            },
            div { class: "flex flex-col px-3 py-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{item.name}"
                    }
                }
                div { class: "flex justify-between text-gray-600",
                    if description.is_empty() {
                        pre { class: "text-xs leading-5 text-gray-600 pt-1", " " }
                    } else {
                        p { class: "text-xs leading-5 text-gray-600 pt-1", "{description}" }
                    }
                }
            }
        }
    }
}
