pub mod sidebar;
pub mod sales_trend;
pub mod table;

// re-exports so callers can use components::Sidebar and components::SalesTrend
pub use sidebar::Sidebar;
pub use sales_trend::SalesTrend;
pub use table::Table;