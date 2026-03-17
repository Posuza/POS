/// Application settings and configuration

use std::sync::OnceLock;

/// Application settings
#[derive(Debug, Clone)]
pub struct AppSettings {
    /// Database file path
    pub database_path: String,
    
    /// JSON data directory path
    pub json_data_path: String,

    /// Images directory path
    pub images_path: String,
    
    /// Log level (debug, info, warn, error)
    pub log_level: String,
    
    /// Application environment (development, staging, production)
    pub environment: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            database_path: "data/pos_data.db".to_string(),
            json_data_path: "data/json".to_string(),
            images_path: "data/images".to_string(),
            log_level: "info".to_string(),
            environment: "production".to_string(),
        }
    }
}

impl AppSettings {
    /// Create new settings with defaults
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Load settings from environment or use defaults
    pub fn from_env() -> Self {
        let mut settings = Self::default();
        
        if let Ok(db_path) = std::env::var("DATABASE_PATH") {
            settings.database_path = db_path;
        }

        if let Ok(json_path) = std::env::var("JSON_DATA_PATH") {
            settings.json_data_path = json_path;
        }
        
        if let Ok(img_path) = std::env::var("IMAGES_PATH") {
            settings.images_path = img_path;
        }
        
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            settings.log_level = log_level;
        }
        
        if let Ok(env) = std::env::var("APP_ENV") {
            settings.environment = env;
        }
        
        settings
    }
    
    /// Check if running in development mode
    pub fn is_development(&self) -> bool {
        self.environment == "development"
    }
    
    /// Check if running in production mode
    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }
}

static SETTINGS: OnceLock<AppSettings> = OnceLock::new();

/// Get global application settings
pub fn get_settings() -> AppSettings {
    // Use `OnceLock` to safely initialize global settings without mutable statics
    SETTINGS
        .get_or_init(|| AppSettings::from_env())
        .clone()
}
