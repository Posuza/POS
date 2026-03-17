use dioxus::prelude::*;

#[component]
pub fn PageContainer(children: Element) -> Element {
    rsx! {
        section { class: "dashboard-v2",
            {children}
        }
    }
}
