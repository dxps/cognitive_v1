use crate::ui::comps::NavUserMenu;
use crate::ui::routes::Route;
use crate::ui::{UiStorage, UI_STATE};
use dioxus::prelude::*;

pub fn Nav() -> Element {
    //
    if *UI_STATE.app_ready.read() == false {
        return rsx! { "" };
    }
    let route = use_route::<Route>().to_string();

    let state = use_context::<Signal<UiStorage>>();
    if state().current_user.is_none() {
        match route.as_str() {
            "/" => render_itself(),
            "/login" => render_itself(),
            "/login-required" => render_itself(),
            "/home" => render_itself(),
            _ => {
                navigator().push(Route::LoginIsRequiredPage {});
                return rsx! {};
            }
        }
    } else {
        return render_itself();
    }
}

fn render_itself() -> Element {
    rsx! {
        nav { class: "absolute w-full px-4 py-2 flex justify-between items-center bg-white z-40",
            Link { class: "text-3xl font-bold leading-none", to: Route::Home {}, Logo {} }
            ul { class: "hidden absolute top-1/2 sm:left-1/3 sm:pl-16 md:left-1/2 lg:left-1/2
                    transform -translate-y-1/2 -translate-x-1/2
                    sm:flex sm:mx-auto sm:flex sm:items-center sm:w-auto sm:space-x-3 lg:space-x-6",
                li {
                    Link {
                        class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200",
                        to: Route::Home {},
                        "Home"
                    }
                }
                NavSep {}
                li {
                    Link {
                        class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200",
                        to: Route::AdminPage {},
                        "Admin"
                    }
                }
            }
            NavUserMenu {}
        }
    }
}

fn NavSep() -> Element {
    rsx! {
        li { class: "text-gray-300",
            div { dangerous_inner_html: r#"
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" class="w-4 h-4 current-fill"
                        viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                            d="M12 5v0m0 7v0m0 7v0m0-13a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 
                               0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
                    </svg>
                "# }
        }
    }
}

fn Logo() -> Element {
    rsx! {
        div {
            img { src: "/assets/logo.png", alt: "logo", class: "h-8" }
        }
    }
}
