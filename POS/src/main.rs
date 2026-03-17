mod config;
mod data;
mod errors;
mod services;
mod ui;
mod utils;

use data::models::user::{User, UserRole};
use dioxus::prelude::*;
use ui::pages::{AdminPage, LoginPage, POSPage};

fn main() {
    if let Err(err) = data::init_json() {
        eprintln!("Failed to initialize JSON data store: {err}");
    }

    // Use dioxus launch entrypoint
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut current_user = use_signal(|| Option::<User>::None);

    let user_opt = current_user.read().clone();

    rsx! {
        div { class: "app",
            style { {ui::styles::get_styles()} }
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
