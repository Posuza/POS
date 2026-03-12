/// Form components

use dioxus::prelude::*;

/// Form input component
#[component]
pub fn FormInput(
    label: String,
    placeholder: String,
    value: String,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "form-group",
            label { "{label}" }
            input { class: "form-input",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| oninput.call(e.value()),
            }
        }
    }
}
