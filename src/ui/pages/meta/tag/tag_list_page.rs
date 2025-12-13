use crate::{
    domain::model::Tag,
    ui::{
        comps::{Breadcrumb, Nav},
        routes::Route,
        UI_STATE,
    },
};
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn TagListPage() -> Element {
    //
    let mut entries = use_signal(|| Arc::new(Vec::new()));
    let mut entries_loaded = use_signal(|| false);

    use_future(move || async move {
        entries.set(UI_STATE.get_tags_list().await);
        entries_loaded.set(true);
    });

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            Breadcrumb { paths: Route::get_path(Route::TagListPage {}) }
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-lg p-3 min-w-[600px] mt-[min(100px)]",
                    div { class: "p-6",
                        div { class: "flex justify-between mb-8",
                            p { class: "text-lg font-medium leading-snug tracking-normal text-gray-500 antialiased",
                                "Tags"
                            }
                            Link {
                                class: "text-gray-500 text-3xl font-extralight hover:text-gray-800 px-2 rounded-xl transition duration-200",
                                to: Route::TagNewPage {},
                                "+"
                            }
                        }
                        if !entries_loaded() {
                            p { class: "pb-4 text-gray-500", "Loading tags ..." }
                        } else {
                            if entries().is_empty() {
                                p { class: "pb-4 text-gray-500", "There are no entries." }
                            }
                            for item in entries().iter() {
                                TagCard { item: item.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TagCard(item: Tag) -> Element {
    rsx! {
        Link { to: Route::TagPage { id: item.id },
            div { class: "flex flex-col px-4 py-2 my-3 bg-white rounded-lg border hover:bg-slate-100 hover:border-slate-100 transition duration-200",
                div { class: "flex justify-between text-gray-600",
                    p { class: "font-medium leading-snug tracking-normal antialiased",
                        "{item.name}"
                    }
                }
                div {
                    class: "flex justify-between text-xs leading-5 text-gray-500 pt-1",
                    dangerous_inner_html: "{item.description.clone().unwrap_or_default()} &nbsp;",
                }
            }
        }
    }
}
