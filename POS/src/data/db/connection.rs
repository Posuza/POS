/// Database connection and pool management

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::Path;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Create/connect to local SQLite database
    let db_path = "pos_data.db";
    
    // Create database file if it doesn't exist
    if !Path::new(db_path).exists() {
        println!("📁 Creating new database: {}", db_path);
    }
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", db_path))
        .await?;
    
    // Run migrations/schema setup
    sqlx::raw_sql(include_str!("../../../schema.sql"))
        .execute(&pool)
        .await?;
    
    println!("✅ Database initialized: {}", db_path);
    
    Ok(pool)
}

pub fn get_db_path() -> String {
    "pos_data.db".to_string()
}
