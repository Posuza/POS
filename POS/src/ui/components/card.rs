/// Card component
use dioxus::prelude::*;

/// Stat card component
#[component]
pub fn Card(icon: String, title: String, value: String) -> Element {
    rsx! {
        div { class: "card",
            span { "{icon}" }
            h3 { "{title}" }
            p { "{value}" }
        }
    }
}
