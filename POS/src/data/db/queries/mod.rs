/// Database queries module

pub mod users;
pub mod products;
pub mod scans;

pub use users::{UserDB, create_user, get_user_by_username, verify_password, get_all_users};
pub use products::{ProductDB, get_product_by_barcode, get_all_products, create_product};
pub use scans::{ScanDB, create_scan, get_scans_for_user, get_today_sales};
