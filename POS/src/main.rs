mod ui;
mod data;
mod config;
mod utils;
mod errors;
mod services;

use dioxus::prelude::*;
use ui::pages::{LoginPage, AdminPage, POSPage};
use data::models::user::{User, UserRole};

fn main() {
    // Use dioxus launch entrypoint
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_user = use_signal(|| Option::<User>::None);

    let user_opt = current_user.read().clone();

    rsx! {
        div { class: "app",
            // Styles (centralized via `ui::styles::get_styles()`)
            style { {ui::styles::get_styles()} }

            // Conditional render: login or the appropriate page
            if let Some(user) = user_opt {
                if user.role == UserRole::Admin {
                    AdminPage { user: user.clone(), on_logout: move |_| current_user.set(None) }
                } else {
                    POSPage { user: user.clone(), on_logout: move |_| current_user.set(None) }
                }
            } else {
                LoginPage { on_login: move |user: User| {
                    println!("on_login: {} role={}", user.username, user.role);
                    current_user.set(Some(user))
                } }
            }
        }
    }
}