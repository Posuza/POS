use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{OnceLock, RwLock};

use crate::config::settings::get_settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub business_name: String,
    pub currency: String,
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            business_name: "My Store".to_string(),
            currency: "USD".to_string(),
        }
    }
}

fn settings_path() -> PathBuf {
    let base = get_settings().json_data_path;
    PathBuf::from(base).join("app_settings.json")
}

fn load_ui_settings() -> UiSettings {
    let path = settings_path();
    if let Ok(contents) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str::<UiSettings>(&contents) {
            return settings;
        }
    }
    UiSettings::default()
}

static UI_SETTINGS: OnceLock<RwLock<UiSettings>> = OnceLock::new();

pub fn get_ui_settings() -> UiSettings {
    let lock = UI_SETTINGS.get_or_init(|| RwLock::new(load_ui_settings()));
    lock.read()
        .map(|s| s.clone())
        .unwrap_or_else(|_| UiSettings::default())
}

pub fn save_ui_settings(settings: &UiSettings) -> std::io::Result<()> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let contents = serde_json::to_string_pretty(settings).unwrap_or_else(|_| "{}".to_string());
    fs::write(&path, contents)?;

    let lock = UI_SETTINGS.get_or_init(|| RwLock::new(settings.clone()));
    if let Ok(mut guard) = lock.write() {
        *guard = settings.clone();
    }
    Ok(())
}

pub fn currency_symbol(code: &str) -> String {
    match code {
        "USD" => "$".to_string(),
        "EUR" => "€".to_string(),
        "GBP" => "£".to_string(),
        "JPY" => "¥".to_string(),
        _ => "".to_string(),
    }
}
