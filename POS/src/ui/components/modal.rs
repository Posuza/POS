/// Modal component
use dioxus::prelude::*;

/// Modal dialog component
#[component]
pub fn Modal() -> Element {
    rsx! {
        div { class: "modal",
            div { class: "modal-content",
                div { class: "modal-header", h3 { "Modal" } }
                div { class: "modal-body", "Content goes here" }
                div { class: "modal-footer",
                    button { class: "btn btn-secondary", "Close" }
                }
            }
        }
    }
}
