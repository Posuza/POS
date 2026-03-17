use super::prelude::*;

#[component]
pub(crate) fn SettingsTab() -> Element {
    let initial_settings = crate::config::ui_settings::get_ui_settings();
    let mut business_name = use_signal(|| initial_settings.business_name.clone());
    let mut currency = use_signal(|| initial_settings.currency.clone());
    let mut show_save_msg = use_signal(|| false);
    let mut save_error = use_signal(|| None::<String>);
    
    let handle_save = move |_| {
        let settings = crate::config::ui_settings::UiSettings {
            business_name: business_name.read().clone(),
            currency: currency.read().clone(),
        };
        match crate::config::ui_settings::save_ui_settings(&settings) {
            Ok(_) => {
                save_error.set(None);
                show_save_msg.set(true);
                spawn(async move {
                    sleep_ms(2_000).await;
                    show_save_msg.set(false);
                });
            }
            Err(err) => {
                save_error.set(Some(format!("Failed to save settings: {}", err)));
            }
        }
    };
    
    rsx! {
        TabContainer {
            h2 { "⚙️ Settings" }
            
            div { class: "settings-form",
                div { class: "form-group",
                    label { "Business Name" }
                    input {
                        placeholder: "Your store name",
                        value: "{business_name}",
                        oninput: move |e| business_name.set(e.value()),
                    }
                }
                
                div { class: "form-group",
                    label { "Currency" }
                    select {
                        value: "{currency}",
                        onchange: move |e| currency.set(e.value()),
                        option { value: "USD", "💵 USD ($)" }
                        option { value: "EUR", "💶 EUR (€)" }
                        option { value: "GBP", "💷 GBP (£)" }
                        option { value: "JPY", "💴 JPY (¥)" }
                    }
                }
                
                div { class: "form-group",
                    label { "Database Location" }
                    p { "📁 data/pos_data.db (Local SQLite)" }
                }
                
                div { class: "form-group",
                    label { "Image Storage" }
                    p { "📁 Products: data/images/products/" }
                    p { style: "margin-top: 5px;", "📁 Profiles: data/images/profiles/" }
                }
                
                if show_save_msg.read().clone() {
                    div { class: "message",
                        "✅ Settings saved successfully!"
                    }
                }
                if let Some(err) = save_error.read().clone() {
                    div { class: "message error",
                        "{err}"
                    }
                }
                
                button {
                    class: "btn btn-primary",
                    onclick: handle_save,
                    "💾 Save Settings"
                }
            }
        }
    }
}
