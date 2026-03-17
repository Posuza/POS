use dioxus::prelude::*;

#[component]
pub fn DashboardContainer(children: Element) -> Element {
    rsx! {
        section { class: "dashboard-v2",
            {children}
        }
    }
}
