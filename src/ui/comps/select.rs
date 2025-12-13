use crate::{domain::model::Id, server::fns::list_entities_by_def_id, ui::pages::Name};

use dioxus::prelude::*;
use indexmap::IndexMap;

#[derive(Clone, Debug, Props, PartialEq)]
pub struct SelectProps {
    pub items: Signal<IndexMap<Id, Name>>,

    pub selected_item_id: Signal<Id>,

    #[props(default = None)]
    pub default_selected_item_id: Option<Id>,

    #[props(default = false)]
    pub disabled: bool,
}

/// The HTML's `select` as a reusable component.<br/>
pub fn Select(props: SelectProps) -> Element {
    //
    // log::debug!("[Select] props: {:?}", props);

    let SelectProps {
        items,
        mut selected_item_id,
        default_selected_item_id,
        disabled,
    } = props;

    log::debug!("[Select] on render, selected_item_id: {:?}", selected_item_id());

    let default_selected_id = match default_selected_item_id {
        Some(id) => id,
        None => Id::default(),
    };

    rsx! {
        if !items().is_empty() {
            select {
                class: "px-3 py-1 min-w-80",
                disabled: "{disabled}",
                multiple: false,
                oninput: move |evt| {
                    selected_item_id.set(evt.value().into());
                    log::debug!("[Select] set selected_item_id: {:?}", evt.value());
                },
                if default_selected_id.is_empty() {
                    option {
                        value: "",
                        selected: "{selected_item_id().is_empty()}",
                        ""
                    }
                }
                for (id , kind) in items() {
                    option {
                        value: "{id}",
                        selected: "{selected_item_id() == id || default_selected_id == id}",
                        "{kind}"
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Props, PartialEq)]
pub struct SelectWithHandlerProps {
    pub items: Signal<IndexMap<Id, Name>>,

    pub selected_item_id: Signal<Id>,

    pub select_handler: EventHandler<()>,

    #[props(default = None)]
    pub default_selected_item_id: Option<Id>,

    #[props(default = false)]
    pub disabled: bool,
}

/// The HTML's `select` as a reusable component.<br/>
pub fn SelectWithHandler(props: SelectWithHandlerProps) -> Element {
    //
    // log::debug!("[SelectWithHandler] props: {:?}", props);

    let SelectWithHandlerProps {
        items,
        mut selected_item_id,
        select_handler,
        default_selected_item_id,
        disabled,
    } = props;

    log::debug!("[SelectWithHandler] on render, selected_item_id: {:?}", selected_item_id());

    let default_selected_id = match default_selected_item_id {
        Some(id) => id,
        None => Id::default(),
    };

    rsx! {
        select {
            class: "px-3 py-1 min-w-80",
            disabled: "{disabled}",
            multiple: false,
            oninput: move |evt| async move {
                selected_item_id.set(evt.value().into());
                log::debug!("[SelectWithHandler] set selected_item_id: {:?}", evt.value());
                if let Ok(source_entities) = list_entities_by_def_id(selected_item_id()).await {
                    log::debug!("[SelectWithHandler] source_entities: {:#?}", source_entities);
                }
                select_handler(());
            },
            if default_selected_id.is_empty() {
                option { value: "", selected: "{selected_item_id().is_empty()}", "" }
            }
            if !items().is_empty() {
                for (id , kind) in items() {
                    option {
                        value: "{id}",
                        selected: "{selected_item_id() == id || default_selected_id == id}",
                        "{kind}"
                    }
                }
            }
        }
    }
}
