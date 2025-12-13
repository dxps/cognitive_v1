use dioxus::prelude::*;

use crate::ui::routes::Route;
use crate::{server::fns::login, ui::comps::Nav};

#[component]
pub fn Login() -> Element {
    //
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut wrong_creds = use_signal(|| false);
    let nav = use_navigator();

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6",
                    div { class: "text-xl mb-6 px-2 text-center text-gray-600", "Login to your account" }
                    div { class: "mt-4 space-y-4",
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "email",
                                r#type: "email",
                                placeholder: "Email address",
                                value: "{email}",
                                autofocus: "true",
                                oninput: move |evt| {
                                    email.set(evt.value());
                                },
                                onmounted: move |evt| async move {
                                    _ = evt.set_focus(true).await;
                                }
                            }
                        }
                        div {
                            input {
                                class: "px-3 py-3 rounded-lg outline-none border-2 focus:border-green-300",
                                name: "password",
                                r#type: "password",
                                placeholder: "Password",
                                value: "{password}",
                                oninput: move |e| {
                                    password.set(e.value());
                                },
                                onkeypress: move |evt| {
                                    async move {
                                        if evt.key() == Key::Enter {
                                            handle_login(email(), password(), &mut wrong_creds, &nav).await;
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "text-center text-red-600 my-8",
                            span { class: if !wrong_creds() { "hidden" }, "Wrong credentials" }
                        }
                        div { class: "text-center my-8",
                            button {
                                class: "bg-blue-50 hover:bg-blue-100 drop-shadow-sm px-4 py-2 rounded-md",
                                onclick: move |_| {
                                    async move {
                                        handle_login(email(), password(), &mut wrong_creds, &nav).await;
                                    }
                                },
                                "Login"
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn handle_login(email: String, password: String, wrong_creds: &mut Signal<bool>, nav: &Navigator) {
    use crate::ui::UiStorage;
    let mut state_sgnl = use_context::<Signal<UiStorage>>();

    match login(format!("{}", email), format!("{}", password)).await {
        Ok(account) => {
            log::debug!(">>> [handle_login] Authenticated and got {:?}. Going to home ...", account);
            let mut state = state_sgnl();
            state.current_user = Some(account);
            state.save_to_localstorage();
            *state_sgnl.write() = state;
            nav.push(Route::Home {});
        }
        Err(e) => {
            log::debug!(">>> [handle_login] Authentication failed. Error: {}", e);
            if e.to_string().contains("wrong credentials") {
                wrong_creds.set(true);
            }
        }
    }
}

pub fn LoginIsRequiredPage() -> Element {
    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-10",
                    div { "You need to login to access this page." }
                    div { class: "mt-6 text-center",
                        Link {
                            class: "text-sm text-gray-600 py-2 px-4 hover:bg-gray-50 rounded-lg transition duration-200",
                            to: Route::Login {},
                            "Login"
                        }
                    }
                }
            }
        }
    }
}
