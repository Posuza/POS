/// Product queries module

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Local;

#[derive(Clone, Debug, sqlx::FromRow, serde::Serialize)]
pub struct ProductDB {
    pub id: String,
    pub barcode: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub category: String,
    pub product_image: Option<String>,        // 📸
    pub product_image_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn get_product_by_barcode(
    db: &SqlitePool,
    barcode: &str,
) -> Result<Option<ProductDB>, sqlx::Error> {
    sqlx::query_as::<_, ProductDB>(
        "SELECT * FROM products WHERE barcode = ?"
    )
    .bind(barcode)
    .fetch_optional(db)
    .await
}

pub async fn get_all_products(db: &SqlitePool) -> Result<Vec<ProductDB>, sqlx::Error> {
    sqlx::query_as::<_, ProductDB>(
        "SELECT * FROM products ORDER BY name ASC"
    )
    .fetch_all(db)
    .await
}

pub async fn create_product(
    db: &SqlitePool,
    barcode: &str,
    name: &str,
    price: f32,
    quantity: i32,
    category: &str,
    image: Option<String>,
    image_type: Option<String>,
) -> Result<ProductDB, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Local::now().to_rfc3339();
    
    sqlx::query_as::<_, ProductDB>(
        "INSERT INTO products (id, barcode, name, price, quantity, category, product_image, product_image_type, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(id)
    .bind(barcode)
    .bind(name)
    .bind(price)
    .bind(quantity)
    .bind(category)
    .bind(image)
    .bind(image_type)
    .bind(&now)
    .bind(&now)
    .fetch_one(db)
    .await
}

pub async fn update_product_image(
    db: &SqlitePool,
    product_id: &str,
    image_base64: &str,
    image_type: &str,
) -> Result<(), sqlx::Error> {
    let now = Local::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE products SET product_image = ?, product_image_type = ?, updated_at = ? WHERE id = ?"
    )
    .bind(image_base64)
    .bind(image_type)
    .bind(now)
    .bind(product_id)
    .execute(db)
    .await?;
    
    Ok(())
}

pub async fn update_product_quantity(
    db: &SqlitePool,
    product_id: &str,
    quantity: i32,
) -> Result<(), sqlx::Error> {
    let now = Local::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE products SET quantity = ?, updated_at = ? WHERE id = ?"
    )
    .bind(quantity)
    .bind(now)
    .bind(product_id)
    .execute(db)
    .await?;
    
    Ok(())
}
