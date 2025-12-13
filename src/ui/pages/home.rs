use dioxus::prelude::*;

use crate::ui::comps::Nav;

const LOGO: Asset = asset!("/assets/cognitive.svg");

#[component]
pub fn Home() -> Element {
    //
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                img { class: "w-[86px] h-[86px] logo_filter", src: LOGO }
                p { class: "text-4xl font-bold text-gray-300", "Cognitive" }
                p { class: "pt-1 text-sm text-gray-400", "A space of knowledge and reasoning." }
            }
        }
    }
}
