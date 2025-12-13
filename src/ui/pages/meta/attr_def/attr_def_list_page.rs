use crate::domain::model::Id;

use crate::{
    domain::model::{AttributeDef, Tag},
    server::fns::list_attribute_defs,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};
use dioxus::prelude::*;
use indexmap::IndexMap;

#[component]
pub fn AttributeDefListPage() -> Element {
    //
    let mut entries = use_signal::<Vec<AttributeDef>>(|| vec![]);

    let mut tags = use_signal(|| IndexMap::new());

    use_future(move || async move {
        tags.set(UI_STATE.get_tags().await);

        if let Ok(attr_defs) = list_attribute_defs().await {
            log::debug!(">>> Got from get_attribute_defs(): {:?}", attr_defs);
            entries.set(attr_defs);
        }
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::AttributeDefListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px]  mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Attributes Definitions"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::AttributeDefNewPage {},
                                "+"
                            }
                        }
                        if entries.is_empty() {
                            p { class: "pb-4 text-gray-500", "There are no entries." }
                        }
                        for attr in entries() {
                            AttrDefCard { attr_def: attr.clone(), tags: tags() }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct AttrDefCardProps {
    pub attr_def: AttributeDef,
    pub tags: IndexMap<Id, Tag>,
}

#[component]
fn AttrDefCard(props: AttrDefCardProps) -> Element {
    //
    let attr_def = props.attr_def;
    let tags = props.tags;

    rsx! {
        Link {
            to: Route::AttributeDefPage {
                attr_def_id: attr_def.id,
            },
            div { class: "flex flex-col p-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600 px-2",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{attr_def.name}"
                    }
                    div { class: "flex",
                        p { class: "text-xs text-slate-500 leading-snug tracking-normal antialiased pr-1",
                            "{attr_def.value_type.label()}"
                        }
                        img { class: "h-4 w-4 mt-px", src: "/assets/struct.png" }
                    }
                }
                div { class: "flex justify-between text-gray-500 px-2",
                    div {
                        class: "flex justify-between text-xs leading-5 text-gray-500 pt-1",
                        dangerous_inner_html: "{attr_def.description.clone().unwrap_or_default()} &nbsp;",
                    }
                    {
                        if attr_def.tag_id.is_some() {
                            let tag_id = attr_def.tag_id.unwrap();
                            match tags.get(&tag_id) {
                                Some(tag) => {
                                    rsx! {
                                        div { class: "flex pt-0.5",
                                            p { class: "text-xs leading-5 pr-1 text-gray-400", "{tag.name.clone()}" }
                                            img { class: "h-4 w-4 mt-0.5", src: "/assets/tag.png" }
                                        }
                                    }
                                }
                                None => {
                                    log::error!(">>> Failed to find tag with id: {}", tag_id);
                                    rsx! {}
                                }
                            }
                        } else {
                            rsx! {
                                p { {attr_def.tag_id.unwrap_or_default().to_string()} }
                            }
                        }
                    }
                }
            }
        }
    }
}
