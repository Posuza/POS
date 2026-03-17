use chrono::Local;
/// Scan queries module
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::FromRow, serde::Serialize)]
pub struct ScanDB {
    pub id: String,
    pub product_id: String,
    pub staff_id: String,
    pub barcode: String,
    pub quantity: i32,
    pub price: Option<f32>,
    pub scanned_at: String,
}

pub async fn create_scan(
    db: &SqlitePool,
    product_id: &str,
    staff_id: &str,
    barcode: &str,
    quantity: i32,
    price: f32,
) -> Result<ScanDB, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Local::now().to_rfc3339();

    sqlx::query_as::<_, ScanDB>(
        "INSERT INTO scans (id, product_id, staff_id, barcode, quantity, price, scanned_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?) 
         RETURNING *",
    )
    .bind(id)
    .bind(product_id)
    .bind(staff_id)
    .bind(barcode)
    .bind(quantity)
    .bind(price)
    .bind(&now)
    .fetch_one(db)
    .await
}

pub async fn get_scans_for_user(
    db: &SqlitePool,
    staff_id: &str,
    limit: i64,
) -> Result<Vec<ScanDB>, sqlx::Error> {
    sqlx::query_as::<_, ScanDB>(
        "SELECT * FROM scans WHERE staff_id = ? ORDER BY scanned_at DESC LIMIT ?",
    )
    .bind(staff_id)
    .bind(limit)
    .fetch_all(db)
    .await
}

pub async fn get_today_sales(db: &SqlitePool) -> Result<(i32, f32), sqlx::Error> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count, SUM(quantity * price) as total 
         FROM scans 
         WHERE DATE(scanned_at) = DATE('now')",
    )
    .fetch_one(db)
    .await?;

    let count: i32 = row.get("count");
    let total: f32 = row.get::<Option<f64>, _>("total").unwrap_or(0.0) as f32;

    Ok((count, total))
}
