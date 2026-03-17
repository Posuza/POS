/// Database module - Connection and query operations

pub mod connection;
pub mod queries;

#[allow(unused_imports)]
pub use connection::{init_db, get_db_path};
#[allow(unused_imports)]
pub use queries::{users, products, scans};
