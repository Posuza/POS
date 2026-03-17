use super::prelude::*;

#[component]
pub(crate) fn PlaceholderTab(title: &'static str) -> Element {
    rsx! {
        div { class: "admin-card",
            div { class: "admin-card-header",
                h3 { "{title}" }
                span { class: "card-subtitle", "This section is ready for data wiring." }
            }
            div { class: "admin-card-body",
                p { "Coming soon." }
            }
        }
    }
}
