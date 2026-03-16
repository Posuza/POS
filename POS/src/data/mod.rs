/// Data module for models, database, and queries

pub mod db;
pub mod models;
pub mod json_store;

#[allow(unused_imports)]
pub use db::*;
#[allow(unused_imports)]
pub use models::*;
#[allow(unused_imports)]
pub use json_store::*;

/// Mod file initialization
pub async fn init() -> crate::errors::Result<()> {
    // Initialize database
    db::connection::init_db().await
        .map(|_pool| ())
        .map_err(|e| crate::errors::PosError::DatabaseError(e.to_string()))
}

/// Initialize JSON store (used when running without DB)
pub fn init_json() -> crate::errors::Result<()> {
    json_store::init_json_store()
}
