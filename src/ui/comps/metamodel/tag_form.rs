use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TagFormProps {
    pub name: Signal<String>,
    pub description: Signal<String>,
    pub action: String,
}

#[component]
pub fn TagForm(props: TagFormProps) -> Element {
    //
    let mut name = props.name;
    let mut description = props.description;
    let action = props.action;

    let is_view = action == "View";
    rsx! {
        div { class: "mt-4 space-y-4",
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Name:" }
                input {
                    key: "name_{action}",
                    class: "px-3 py-1 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    r#type: "text",
                    name: "name_{action}",
                    value: "{name}",
                    maxlength: 64,
                    readonly: is_view,
                    autofocus: !is_view,
                    oninput: move |evt| {
                        name.set(evt.value());
                    },
                    onmounted: move |evt| async move {
                        if !is_view {
                            _ = evt.set_focus(true).await;
                        }
                    }
                }
            }
            div { class: "flex",
                label { class: "pr-3 py-2 min-w-28", "Description:" }
                textarea {
                    class: "px-3 py-2 rounded-lg outline-none border-1 focus:border-green-300 min-w-80",
                    rows: 4,
                    cols: 32,
                    placeholder: "an optional description",
                    value: "{description}",
                    readonly: is_view,
                    maxlength: 256,
                    oninput: move |evt| {
                        description.set(evt.value());
                    }
                }
            }
        }
    }
}
