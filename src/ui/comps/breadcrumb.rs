use crate::ui::{comps::GtSep, routes::Route};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct BreadcrumbProps {
    pub paths: Vec<(String, Route)>,
}

pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    //
    let last_idx = props.paths.len() - 1;
    rsx! {
        div { class: "absolute left-1/2 -translate-x-1/2 font-[sans-serif] flex items-center mt-16 mb-4 z-40",
            ul { class: "flex items-center justify-center space-x-2",
                for (i , (label , route)) in props.paths.into_iter().enumerate() {
                    Link { class: "text-gray-500 text-xs cursor-pointer", to: route, "{label}" }
                    if i < last_idx {
                        GtSep {}
                    }
                }
            }
        }
    }
}
