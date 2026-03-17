use crate::data::json_store::get_store;
use crate::data::models::user::{User, UserRole};
use dioxus::events::Key;
use dioxus::prelude::*;

#[component]
pub fn LoginPage(on_login: EventHandler<User>) -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut selected_role = use_signal(|| UserRole::Staff);
    let mut error = use_signal(|| Option::<String>::default());
    let mut is_loading = use_signal(|| false);

    let mut handle_login = move |_| {
        if username.read().is_empty() {
            error.set(Some("Please enter a username".to_string()));
            return;
        }

        is_loading.set(true);
        error.set(None);

        let store = get_store();
        let username_value = username.read().clone();

        let user_record = match store.find_user_by_username(&username_value) {
            Some(u) => u,
            None => {
                is_loading.set(false);
                error.set(Some("User not found".to_string()));
                return;
            }
        };

        if user_record.status != "active" {
            is_loading.set(false);
            error.set(Some("User is inactive".to_string()));
            return;
        }

        if *selected_role.read() != user_record.role {
            is_loading.set(false);
            error.set(Some("Selected role does not match user role".to_string()));
            return;
        }

        let user = user_record.to_user();

        is_loading.set(false);
        // Debug: print login attempt
        println!(
            "Login attempt: {} role={}",
            username.read(),
            selected_role.read()
        );
        on_login.call(user);
    };

    rsx! {
        div { class: "login-page",
            div { class: "login-container",
                div { class: "login-box",
                    // Header
                    div { class: "login-header",
                        h1 { "POS System" }
                        p { "Offline Point of Sale" }
                    }

                    // Role selector
                    div { class: "role-selector",
                        h3 { "Login As:" }

                        div { class: "role-buttons",
                            button {
                                class: if *selected_role.read() == UserRole::Staff { "role-btn active" } else { "role-btn" },
                                onclick: move |_| selected_role.set(UserRole::Staff),
                                "Staff (Cashier)"
                            }

                            button {
                                class: if *selected_role.read() == UserRole::Admin { "role-btn active" } else { "role-btn" },
                                onclick: move |_| selected_role.set(UserRole::Admin),
                                "Admin"
                            }
                        }
                    }

                    // Form
                    div { class: "login-form",
                        div { class: "form-group",
                            label { "Username" }
                            input {
                                class: "form-input",
                                placeholder: "Enter username",
                                value: "{username}",
                                r#type: "text",
                                oninput: move |e| username.set(e.value()),
                                disabled: is_loading,
                            }
                        }

                        div { class: "form-group",
                            label { "Password" }
                            input {
                                class: "form-input",
                                placeholder: "Enter password",
                                value: "{password}",
                                r#type: "password",
                                oninput: move |e| password.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        handle_login(());
                                    }
                                },
                                disabled: is_loading,
                            }
                        }

                        if let Some(err) = error.read().clone() {
                            div { class: "error-message",
                                "❌ {err}"
                            }
                        }

                        button {
                            class: "btn btn-primary btn-large",
                            onclick: move |_| handle_login(()),
                            disabled: is_loading,
                            "Login"
                        }
                    }

                    // Demo info
                    div { class: "demo-box",
                        h4 { "Demo Accounts:" }
                        p { "staff / (any password)" }
                        p { "admin / (any password)" }
                    }
                }
            }
        }
    }
}
