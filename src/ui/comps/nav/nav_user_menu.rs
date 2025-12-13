use dioxus::prelude::*;

use crate::server::fns::has_admin_permissions;
use crate::ui::routes::Route;
use crate::ui::{UiStorage, UI_STATE};

pub fn NavUserMenu() -> Element {
    //
    let state = use_context::<Signal<UiStorage>>();
    let mut show_dropdown = use_signal(|| false);

    if *UI_STATE.app_ready.read() == false {
        return rsx! {};
    };

    if state().current_user.is_none() {
        log::debug!(">>> [NavUserMenu] There is no locally saved user.");
        rsx! {
            Link {
                class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-100 rounded-lg transition duration-200 sm:inline-block sm:ml-auto sm:mr-3",
                to: Route::Login {},
                "Login"
            }
        }
    } else {
        let username = state().current_user.unwrap().username;
        log::trace!(">>> [NavUserMenu] There is a locally saved user with username: {:?}.", username);
        rsx! {
            div { class: "text-sm text-gray-600 hover:bg-gray-100 rounded-lg transition duration-200 flex flex-col items-end overflow-visible",
                button {
                    class: "px-8 py-2 align rounded-lg text-sm outline-none",
                    onclick: move |_| {
                        let curr_val = show_dropdown();
                        *show_dropdown.write() = !curr_val;
                    },
                    div {
                        class: "rounded-full justify-center",
                        dangerous_inner_html: user_icon()
                    }
                }
                if show_dropdown() {
                    NavUserDropdown { username, show_dropdown }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavUserDropdownProps {
    username: String,
    show_dropdown: Signal<bool>,
}

fn NavUserDropdown(mut props: NavUserDropdownProps) -> Element {
    //
    let has_admin_perms = use_resource(move || async move { has_admin_permissions().await });

    rsx! {
        div {
            "style": "width: 100%; height: 1000%; padding: 0; position: absolute; top: 0; left: 0",
            onclick: move |_| {
                log::debug!(">>> [NavUserDropdown] Clicked in the outer div.");
                *props.show_dropdown.write() = false;
            },
            div { class: "w-20 mt-14 mr-[72px] bg-white rounded-lg shadow-2xl float-right",
                div {
                    ul { class: "shadow-2xl bg-white py-2 z-[1000] min-w-full w-max rounded-lg max-h-96 overflow-auto",
                        li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                                to: Route::UserProfilePage {
                                    username: props.username,
                                },
                                div {
                                    class: "mr-3",
                                    dangerous_inner_html: user_icon()
                                }
                                "  My profile"
                            }
                        }
                        {
                            match &*has_admin_perms.read_unchecked() {
                                Some(Ok(true)) => rsx! {
                                    li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                                        Link {
                                            class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                                            to: Route::AdminPage {},
                                            div { class: "mr-3", dangerous_inner_html: admin_icon() }
                                            "  Admin Area"
                                        }
                                    }
                                },
                                _ => rsx! {},

                            }
                        },
                        li { class: "px-4 py-2",
                            hr {}
                        }
                        li { class: "flex items-center text-[#333] hover:bg-gray-100 hover:text-orange-600 text-sm cursor-pointer",
                            Link {
                                class: "py-2.5 px-5 min-w-full w-max min-h-full flex text-[#333]",
                                to: Route::Logout {},
                                div {
                                    class: "mr-3",
                                    dangerous_inner_html: logout_icon()
                                }
                                "  Logout"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn user_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4" viewBox="0 0 512 512">
        <path
            d="M337.711 241.3a16 16 0 0 0-11.461 3.988c-18.739 16.561-43.688 25.682-70.25 
               25.682s-51.511-9.121-70.25-25.683a16.007 16.007 0 0 0-11.461-3.988c-78.926 4.274-140.752 
               63.672-140.752 135.224v107.152C33.537 499.293 46.9 512 63.332 512h385.336c16.429 0 29.8-12.707 
               29.8-28.325V376.523c-.005-71.552-61.831-130.95-140.757-135.223zM446.463 480H65.537V376.523c0-52.739 
               45.359-96.888 104.351-102.8C193.75 292.63 224.055 302.97 256 302.97s62.25-10.34 86.112-29.245c58.992 5.91 
               104.351 50.059 104.351 102.8zM256 234.375a117.188 117.188 0 1 0-117.188-117.187A117.32 
               117.32 0 0 0 256 234.375zM256 32a85.188 85.188 0 1 1-85.188 85.188A85.284 85.284 0 0 1 256 32z"
            data-original="\#000000"></path>
    </svg>
    "#
    .to_string()
}

fn admin_icon() -> String {
    r#"
    <svg fill="\#000000" class="w-4 h-4" version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" 
    xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 512 512" xml:space="preserve">
        <g>
            <g>
                <path d="M235.082,392.745c-5.771,0-10.449,4.678-10.449,10.449v4.678c0,5.771,4.678,10.449,10.449,10.449
                    c5.77,0,10.449-4.678,10.449-10.449v-4.678C245.531,397.423,240.853,392.745,235.082,392.745z"/>
            </g>
        </g>
        <g>
            <g>
                <path d="M492.948,313.357l-31.393-25.855c1.58-10.4,2.38-20.968,2.38-31.502c0-10.534-0.8-21.104-2.381-31.504l31.394-25.856
                    c10.032-8.262,12.595-22.42,6.099-33.66L456.35,91.029c-4.704-8.173-13.479-13.25-22.903-13.25c-3.19,0-6.326,0.573-9.302,1.695
                    l-38.109,14.274c-16.546-13.286-34.848-23.869-54.55-31.54l-6.683-40.082C322.676,9.306,311.701,0,298.704,0h-85.408
                    C200.3,0,189.324,9.307,187.2,22.119l-6.684,40.088c-19.703,7.673-38.007,18.255-54.553,31.542L87.898,79.492
                    c-2.999-1.138-6.14-1.715-9.338-1.715c-9.414,0-18.191,5.074-22.903,13.241l-42.702,73.96
                    c-6.499,11.244-3.935,25.403,6.097,33.664l31.394,25.855c-1.58,10.4-2.38,20.969-2.38,31.503c0,10.534,0.8,21.103,2.38,31.503
                    l-31.394,25.856c-10.032,8.262-12.595,22.42-6.099,33.66l42.703,73.963c4.716,8.171,13.492,13.247,22.904,13.247
                    c3.205,0,6.352-0.581,9.294-1.703l38.107-14.275c16.547,13.287,34.85,23.87,54.551,31.541l6.682,40.075
                    C189.316,502.692,200.293,512,213.297,512h85.408c12.991,0,23.967-9.304,26.096-22.118l6.683-40.089
                    c19.705-7.673,38.008-18.255,54.554-31.542l38.07,14.261c2.999,1.137,6.141,1.713,9.336,1.713c9.411,0,18.185-5.074,22.9-13.241
                    l42.703-73.962C505.543,335.776,502.979,321.619,492.948,313.357z M298.704,491.102H245.53v-49.427
                    c0-5.771-4.678-10.449-10.449-10.449c-5.771,0-10.449,4.678-10.449,10.449v49.427h-10.922V376.504
                    c13.606,4.844,28.061,7.375,42.865,7.382c0.003,0,0.066,0,0.07,0c14.852,0,29.325-2.528,42.928-7.376v114.515h0.001
                    C299.289,491.069,299,491.102,298.704,491.102z M256.642,362.988h-0.057c-58.964-0.029-106.933-48.026-106.932-106.99
                    c0.001-34.314,16.175-65.814,43.158-85.745v76.229c0,3.671,1.926,7.072,5.073,8.96l53.381,32.027
                    c3.309,1.984,7.443,1.984,10.752,0l53.381-32.027c3.147-1.889,5.073-5.29,5.073-8.96v-76.229
                    c26.983,19.931,43.159,51.432,43.157,85.747c0,28.528-11.143,55.382-31.374,75.614
                    C312.022,351.846,285.169,362.988,256.642,362.988z M480.949,336.57l-42.705,73.965c-1.326,2.296-4.122,3.423-6.769,2.42
                    l-43.772-16.397c-3.557-1.331-7.555-0.63-10.444,1.834c-16.925,14.428-36.026,25.589-56.79,33.212v-64.779
                    c9.585-5.551,18.513-12.386,26.56-20.434c24.179-24.18,37.495-56.281,37.495-90.391c0.001-48.242-26.729-91.831-69.76-113.754
                    c-3.239-1.651-7.103-1.498-10.203,0.401c-3.099,1.9-4.989,5.274-4.989,8.909v89.011l-42.932,25.759l-42.932-25.759v-89.011
                    c0-3.635-1.89-7.009-4.989-8.909c-3.099-1.899-6.963-2.05-10.203-0.401c-43.03,21.922-69.76,65.51-69.762,113.752
                    c-0.001,34.743,13.583,67.154,38.247,91.26c7.858,7.68,16.53,14.23,25.809,19.585v65.235
                    c-21.258-7.63-40.795-18.958-58.071-33.684c-1.922-1.638-4.333-2.497-6.78-2.497c-1.232,0-2.473,0.217-3.663,0.664l-43.83,16.419
                    c-0.613,0.234-1.255,0.353-1.905,0.353c-1.969,0-3.81-1.071-4.805-2.796l-42.706-73.968c-1.365-2.361-0.822-5.337,1.288-7.076
                    L68.389,299.8c2.926-2.411,4.318-6.216,3.635-9.944c-2.03-11.12-3.061-22.509-3.061-33.856c0-11.346,1.03-22.736,3.063-33.854
                    c0.681-3.729-0.709-7.535-3.636-9.944l-36.051-29.691c-2.112-1.74-2.654-4.716-1.287-7.08l42.705-73.966
                    c1.323-2.294,4.109-3.429,6.769-2.419l43.772,16.396c3.555,1.33,7.554,0.63,10.444-1.833
                    c17.417-14.847,37.129-26.244,58.589-33.876c3.576-1.272,6.182-4.382,6.805-8.126l7.679-46.059
                    c0.446-2.694,2.752-4.649,5.482-4.649h85.408c2.73,0,5.036,1.955,5.485,4.656l7.679,46.053c0.624,3.744,3.23,6.856,6.806,8.126
                    c21.459,7.631,41.17,19.027,58.586,33.874c2.89,2.463,6.888,3.165,10.444,1.833l43.794-16.405c0.631-0.238,1.287-0.358,1.95-0.358
                    c1.97,0,3.805,1.064,4.798,2.789l42.706,73.967c1.365,2.361,0.822,5.337-1.288,7.076l-36.052,29.692
                    c-2.926,2.411-4.318,6.215-3.635,9.944c2.03,11.118,3.061,22.509,3.061,33.855c0,11.346-1.03,22.735-3.063,33.853
                    c-0.681,3.728,0.709,7.535,3.636,9.944l36.051,29.691C481.774,331.227,482.316,334.205,480.949,336.57z"/>
            </g>
        </g>
    </svg>
    "#
    .to_string()
}

fn logout_icon() -> String {
    r#"
    <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="w-4 h-4"
        viewBox="0 0 6.35 6.35">
        <path
            d="M3.172.53a.265.266 0 0 0-.262.268v2.127a.265.266 0 0 0 .53 0V.798A.265.266 0 0 0 3.172.53zm1.544.532a.265.266 
               0 0 0-.026 0 .265.266 0 0 0-.147.47c.459.391.749.973.749 1.626 0 1.18-.944 2.131-2.116 2.131A2.12 2.12 0 0 1 
               1.06 3.16c0-.65.286-1.228.74-1.62a.265.266 0 1 0-.344-.404A2.667 2.667 0 0 0 .53 3.158a2.66 2.66 0 0 0 2.647 
               2.663 2.657 2.657 0 0 0 2.645-2.663c0-.812-.363-1.542-.936-2.03a.265.266 0 0 0-.17-.066z"
            data-original="\#000000"></path>
    </svg>
    "#
    .to_string()
}
