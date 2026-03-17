use dioxus::prelude::*;

#[component]
pub fn TabContainer(children: Element) -> Element {
    rsx! {
        div { class: "tab-content",
            {children}
        }
    }
}
