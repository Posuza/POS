pub mod products;
pub mod scans;
/// Database queries module
pub mod users;

pub use products::{create_product, get_all_products, get_product_by_barcode, ProductDB};
pub use users::{create_user, get_all_users, get_user_by_username, verify_password, UserDB};
// ...existing code...
