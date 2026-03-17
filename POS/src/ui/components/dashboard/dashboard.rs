use dioxus::prelude::*;

#[component]
pub fn DashboardContainer(children: Element) -> Element {
    rsx! {
        div { class: "dashboard-v2",
            {children}
        }
    }
}
