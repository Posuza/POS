/// Database module - Connection and query operations
pub mod connection;
pub mod queries;

#[allow(unused_imports)]
pub use connection::{get_db_path, init_db};
#[allow(unused_imports)]
pub use queries::{products, scans, users};
