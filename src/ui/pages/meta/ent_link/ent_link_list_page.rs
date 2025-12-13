use dioxus::prelude::*;

use crate::{
    domain::model::EntityLink,
    server::fns::list_entity_links,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};

#[component]
pub fn EntityLinkListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<(EntityLink, String, String)>>(|| vec![]);

    use_future(move || async move {
        match list_entity_links().await {
            Ok(items) => {
                UI_STATE.get_ent_defs().await;
                UI_STATE.get_ent_link_def_list().await;
                let entries_tuples = items
                    .into_iter()
                    .map(|item| {
                        let ent_link_def = UI_STATE.get_ent_link_def_sync(&item.def_id).unwrap();
                        let source_ent_def = UI_STATE.get_ent_def_sync(&ent_link_def.source_entity_def_id).unwrap();
                        let target_ent_def = UI_STATE.get_ent_def_sync(&ent_link_def.target_entity_def_id).unwrap();
                        (item, source_ent_def.name, target_ent_def.name)
                    })
                    .collect();
                entries.set(entries_tuples);
            }
            Err(e) => {
                // TODO: Capture the error and display it.
                log::error!("Failed to list entity links: {}", e)
            }
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::EntityLinkListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Entity Links"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::EntityLinkNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for item in entries() {
                            EntityLinkCard {
                                item: item.0,
                                source_ent_def_name: item.1,
                                target_ent_def_name: item.2,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntityLinkCard(item: EntityLink, source_ent_def_name: String, target_ent_def_name: String) -> Element {
    //
    rsx! {
        Link {
            to: Route::EntityLinkPage {
                id: item.id,
            },
            div { class: "flex flex-col px-3 py-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    div {
                        p { class: "text-sm leading-snug tracking-normal antialiased",
                            Link {
                                to: Route::EntityPage {
                                    id: item.source_entity_id.clone(),
                                },
                                onclick: move |evt: Event<MouseData>| evt.stop_propagation(),
                                "{source_ent_def_name} ({item.source_entity_id})"
                            }
                            " → "
                            Link {
                                to: Route::EntityPage {
                                    id: item.target_entity_id.clone(),
                                },
                                onclick: move |evt: Event<MouseData>| evt.stop_propagation(),
                                "{target_ent_def_name} ({item.target_entity_id})"
                            }
                        }
                    }
                    div { class: "flex",
                        p { class: "mt-1 text-xs leading-snug tracking-normal antialiased pr-1",
                            "{item.kind}"
                        }
                        img { class: "h-4 w-4 mt-1", src: "/assets/struct.png" }
                    }
                }
            }
        }
    }
}
