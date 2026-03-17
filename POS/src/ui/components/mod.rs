/// UI Components module

pub mod navbar;
pub mod button;
pub mod form;
pub mod modal;
pub mod card;
pub mod tab_container;

// the actual submodules live in the `dashboard` directory
pub mod dashboard;

// re-exports
pub use navbar::Navbar;
pub use button::Button;
pub use form::FormInput;
pub use modal::Modal;
pub use card::Card;
pub use tab_container::TabContainer;

// re-export sidebar and sales_trend from the dashboard submodule
pub use dashboard::sidebar::Sidebar;
pub use dashboard::sales_trend::SalesTrend;
// re-export dashboard table component
pub use dashboard::table::Table;
pub use dashboard::dashboard::DashboardContainer;
