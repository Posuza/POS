/// UI Components module

pub mod navbar;
pub mod button;
pub mod form;
pub mod modal;
pub mod card;

// the actual submodules live in the `dashborads` directory
pub mod dashborads;

// re-exports
pub use navbar::Navbar;
pub use button::Button;
pub use form::FormInput;
pub use modal::Modal;
pub use card::Card;

// re-export sidebar and sales_trend from the dashborads submodule
pub use dashborads::sidebar::Sidebar;
pub use dashborads::sales_trend::SalesTrend;
// re-export dashboard table component
pub use dashborads::table::Table;
