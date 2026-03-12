/// Database module - Connection and query operations

pub mod connection;
pub mod queries;

pub use connection::{init_db, get_db_path};
pub use queries::{users, products, scans};
