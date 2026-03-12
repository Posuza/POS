/// Sidebar component

use dioxus::prelude::*;

/// Sidebar menu component (reusable)
use dioxus::prelude::*;

#[component]
pub fn Sidebar(active: String, on_select: EventHandler<String>, header: String) -> Element {
    rsx! {
        div { class: "sidebar",
            div { class: "sidebar-header", {header} }
            ul { class: "sidebar-menu",
                li { a { class: if active == "dashboard" { "active" } else { "" }, href: "#", onclick: move |_| on_select.call("dashboard".to_string()), span { class: "menu-icon", "📊" }, span { class: "menu-label", "Dashboard" } } }
                li { a { class: if active == "sales" { "active" } else { "" }, href: "#", onclick: move |_| on_select.call("sales".to_string()), span { class: "menu-icon", "💸" }, span { class: "menu-label", "Sales" } } }
                li { a { class: if active == "products" { "active" } else { "" }, href: "#", onclick: move |_| on_select.call("products".to_string()), span { class: "menu-icon", "📦" }, span { class: "menu-label", "Products" } } }
                li { a { class: if active == "staff" { "active" } else { "" }, href: "#", onclick: move |_| on_select.call("staff".to_string()), span { class: "menu-icon", "👥" }, span { class: "menu-label", "Staff" } } }
                li { a { class: if active == "settings" { "active" } else { "" }, href: "#", onclick: move |_| on_select.call("settings".to_string()), span { class: "menu-icon", "⚙️" }, span { class: "menu-label", "Settings" } } }
            }
        }
    }
}
