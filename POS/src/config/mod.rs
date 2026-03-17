/// Configuration module
/// 
/// Handles application-wide configuration, constants, and settings

pub mod constants;
pub mod settings;
pub mod ui_settings;

#[allow(unused_imports)]
pub use constants::*;
#[allow(unused_imports)]
pub use settings::AppSettings;
#[allow(unused_imports)]
pub use ui_settings::UiSettings;
