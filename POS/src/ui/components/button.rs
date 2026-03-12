/// Button component

use dioxus::prelude::*;

/// Reusable button component
#[component]
pub fn Button(label: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        button { 
            class: "btn",
            onclick: move |_| onclick.call(()),
            "{label}"
        }
    }
}
