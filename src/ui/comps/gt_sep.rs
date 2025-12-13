use dioxus::prelude::*;

/// ">" (symbol) separator.
pub fn GtSep() -> Element {
    rsx! {
        div {
            class: "pl-[6px] pb-1",
            dangerous_inner_html: r##"
        <svg xmlns="http://www.w3.org/2000/svg" class="fill-gray-400 w-3.5 -rotate-90 text-gray-300" viewBox="0 0 32 32">
          <path fill-rule="evenodd"
            d="M11.99997 18.1669a2.38 2.38 0 0 1-1.68266-.69733l-9.52-9.52a2.38 2.38 0 1 1 3.36532-3.36532l7.83734 7.83734 7.83734-7.83734a2.38 2.38 0 1 1 3.36532 3.36532l-9.52 9.52a2.38 2.38 0 0 1-1.68266.69734z"
            clip-rule="evenodd" data-original="#000000"></path>
        </svg>
        "##
        }
    }
}
