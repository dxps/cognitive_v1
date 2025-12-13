use dioxus::prelude::*;

use crate::{
    domain::model::EntityDef,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};

#[component]
pub fn EntityDefListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<EntityDef>>(|| vec![]);

    use_future(move || async move {
        entries.set(UI_STATE.get_ent_defs_list().await);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entities Definitions"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityDefNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for ed in entries() {
                            EntityDefCard { ent_def: ed.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityDefCard(ent_def: EntityDef) -> Element {
    //
    let description = match ent_def.description {
        Some(description) => description,
        None => String::from(""),
    };
    rsx! {
        Link {
            to: Route::EntityDefPage {
                id: ent_def.id,
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{ent_def.name}"
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
