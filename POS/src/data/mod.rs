/// Data module for models, database, and queries

pub mod db;
pub mod models;

#[allow(unused_imports)]
pub use db::*;
#[allow(unused_imports)]
pub use models::*;

/// Mod file initialization
pub async fn init() -> crate::errors::Result<()> {
    // Initialize database
    db::connection::init_db().await
        .map(|_pool| ())
        .map_err(|e| crate::errors::PosError::DatabaseError(e.to_string()))
}
