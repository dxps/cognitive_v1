use crate::domain::model::UserAccount;
use crate::server::fns::{save_user_profile_primary_info, set_user_profile_new_password};
use crate::ui::comps::Nav;
use crate::ui::pages::LoginIsRequiredPage;
use crate::ui::{UiStorage, UI_STATE};
use dioxus::prelude::*;

#[component]
pub fn UserProfilePage(username: String) -> Element {
    //
    if *UI_STATE.app_ready.read() == false {
        return rsx! { "Loading..." };
    }
    let state = use_context::<Signal<UiStorage>>();
    if state().current_user.is_none() {
        rsx! {
            LoginIsRequiredPage {}
        }
    } else {
        render_user_profile_page(username, state().current_user.unwrap())
    }
}

fn render_user_profile_page(username: String, ua: UserAccount) -> Element {
    //
    log::debug!(">>> [render_user_profile_page] Username: {}, UserAccount: {:?}", username, ua);
    let mut tab_to_show = use_signal(|| "primary_info".to_string());

    rsx! {
        div { class: "flex flex-col min-h-screen bg-gray-100",
            Nav {}
            div { class: "flex flex-col min-h-screen justify-center items-center drop-shadow-2xl",
                div { class: "bg-white rounded-md p-6 mt-24 mb-8 min-w-[600px]",
                    h1 { class: "text-3xl text-[#333] font-bold text-center",
                        if username == ua.username {
                            { "My Profile" }
                        } else {
                            { format!("{}'s Profile", username) }
                        }
                    }
                    // The tabs.
                    ul { class: "flex gap-4 bg-gray-100 rounded-lg my-4 p-[3.4px] w-max overflow-hidden font-sans mx-auto",
                        li {
                            class: if tab_to_show() == "primary_info".to_string() {
                                "text-green-600 rounded-lg font-semibold text-center text-sm bg-white py-2 px-4 tracking-wide cursor-pointer"
                            } else {
                                "text-gray-600 rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer"
                            },
                            onclick: move |_| tab_to_show.set("primary_info".to_string()),
                            "Primary Info"
                        }
                        li {
                            class: if tab_to_show() == "security".to_string() {
                                "text-green-600 rounded-lg font-semibold text-center text-sm bg-white py-2 px-4 tracking-wide cursor-pointer"
                            } else {
                                "text-gray-600 rounded-lg text-center text-sm hover:bg-white hover:text-lilac py-2 px-4 tracking-wide cursor-pointer"
                            },
                            onclick: move |_| tab_to_show.set("security".to_string()),
                            "Security"
                        }
                    }
                    if tab_to_show() == "primary_info".to_string() {
                        PrimaryInfo { ua }
                    } else if tab_to_show() == "security".to_string() {
                        Security { ua }
                    }
                }
            }
        }
    }
}

#[component]
fn PrimaryInfo(ua: UserAccount) -> Element {
    //
    let mut username = use_signal(|| ua.username.clone());
    let mut email = use_signal(|| ua.email.clone());
    let mut bio = use_signal(|| ua.bio.clone());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut saved = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6",
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Username" }
                input {
                    class: "w-full",
                    r#type: "text",
                    placeholder: "Username",
                    value: "{ua.username}",
                    maxlength: 48,
                    oninput: move |evt| { username.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Email" }
                input {
                    class: "w-full rounded-md py-2.5",
                    r#type: "text",
                    placeholder: "Email",
                    value: "{ua.email}",
                    maxlength: 64,
                    oninput: move |evt| { email.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Biography" }
                textarea {
                    class: "w-full rounded-md py-2.5 px-3",
                    cols: 64,
                    rows: 6,
                    placeholder: "Biography",
                    value: "{ua.bio}",
                    maxlength: 1024,
                    oninput: move |evt| { bio.set(evt.value()) }
                }
            }
            div { class: "text-center my-8",
                button {
                    class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                    onclick: move |_| {
                        let mut ua = ua.clone();
                        async move {
                            match save_user_profile_primary_info(
                                    ua.id.clone(),
                                    username(),
                                    email(),
                                    bio(),
                                )
                                .await
                            {
                                Ok(_) => {
                                    err.set(None);
                                    saved.set(true);
                                    ua.username = username();
                                    ua.email = email();
                                    ua.bio = bio();
                                    let mut state_sgnl = use_context::<Signal<UiStorage>>();
                                    let mut state = state_sgnl();
                                    state.current_user = Some(ua);
                                    state.save_to_localstorage();
                                    *state_sgnl.write() = state;
                                }
                                Err(e) => {
                                    err.set(Some(e.to_string()));
                                    saved.set(false);
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
            // Show the button's action result in the UI.
            if err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {err().unwrap()} }
                }
            } else if saved() {
                div { class: "text-center text-green-600 my-8",
                    span { { "Successfully updated" } }
                }
            }
        }
    }
}

#[component]
fn Security(ua: UserAccount) -> Element {
    //
    let mut curr_password = use_signal(|| String::new());
    let mut new_password = use_signal(|| String::new());
    let mut confirm_password = use_signal(|| String::new());
    let mut result_err: Signal<Option<String>> = use_signal(|| None);
    let mut result_ok = use_signal(|| false);

    rsx! {
        div { class: "mt-8 space-y-6",
            div { class: "flex flex-row text-sm text-gray-500",
                { "Id: " },
                { ua.id.to_string() }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Current Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the current password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { curr_password.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "New Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the new password",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { new_password.set(evt.value()) }
                }
            }
            div {
                label { class: "text-sm text-gray-500 block mb-2", "Confirm New Password" }
                input {
                    class: "w-full",
                    r#type: "password",
                    placeholder: "Enter the new password again",
                    value: "",
                    maxlength: 48,
                    oninput: move |evt| { confirm_password.set(evt.value()) }
                }
            }
            div { class: "text-center my-8",
                button {
                    class: "bg-gray-100 hover:bg-green-100 drop-shadow-sm px-4 py-2 rounded-md",
                    onclick: move |_| {
                        let ua = ua.clone();
                        async move {
                            log::debug!(
                                ">>> [Security] Received: curr_password: {}, new_password: {}, confirm_password: {}",
                                curr_password(), new_password(), confirm_password()
                            );
                            if new_password().is_empty() || new_password() != confirm_password() {
                                result_err
                                    .set(
                                        Some(
                                            "The new password and confirm password do not match.".into(),
                                        ),
                                    );
                                return;
                            }
                            match set_user_profile_new_password(
                                    ua.id.clone(),
                                    curr_password(),
                                    new_password(),
                                )
                                .await
                            {
                                Ok(res) => {
                                    match res {
                                        Ok(_) => {
                                            result_err.set(None);
                                            result_ok.set(true);
                                        }
                                        Err(e) => {
                                            result_err.set(Some(format!("Error: {}", e.to_string())));
                                            result_ok.set(false);
                                        }
                                    }
                                }
                                Err(e) => {
                                    result_err.set(Some(e.to_string()));
                                }
                            }
                        }
                    },
                    "Update"
                }
            }
            // Show the button's action result in the UI.
            if result_err().is_some() {
                div { class: "text-center text-red-600 my-8",
                    span { {result_err().unwrap()} }
                }
            } else if result_ok() {
                div { class: "text-center text-green-600 my-8",
                    span { { "Successfully updated" } }
                }
            }
        }
    }
}
