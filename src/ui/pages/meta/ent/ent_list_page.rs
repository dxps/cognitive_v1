use dioxus::prelude::*;

use crate::{
    domain::model::Entity,
    server::fns::list_entities,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};

#[component]
pub fn EntityListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<Entity>>(|| vec![]);

    use_future(move || async move {
        UI_STATE.get_ent_defs_list().await;
        if let Ok(entitites) = list_entities().await {
            entries.set(entitites);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entities"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for e in entries() {
                            EntityCard { ent: e.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityCard(ent: Entity) -> Element {
    //
    log::debug!("[EntityCard] ent: {:?}", ent);
    rsx! {
        Link {
            to: Route::EntityPage {
                id: ent.id.clone(),
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded-md border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-500",
                    div {
                        p { class: "text-lg leading-5 text-gray-600 font-medium pt-1 pl-2",
                            "{ent.listing_attr_value}"
                        }
                        p { class: "text-xs leading-5 text-gray-400 pt-1 pl-2",
                            "({ent.listing_attr_name})"
                        }
                    }
                    div { class: "flex",
                        p { class: "mt-1 text-xs leading-snug tracking-normal antialiased pr-1",
                            "{ent.kind}"
                        }
                        img { class: "h-4 w-4 mt-1", src: "/assets/struct.png" }
                    }
                }
            }
        }
    }
}
