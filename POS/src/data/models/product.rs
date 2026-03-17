use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub barcode: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub category: String,
    pub product_image: Option<String>,      // 📸 Base64 or path
    pub product_image_type: Option<String>, // jpeg, png
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub barcode: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub category: String,
    pub product_image: Option<String>, // 📸 Base64
    pub product_image_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub price: Option<f32>,
    pub quantity: Option<i32>,
    pub product_image: Option<String>,
    pub product_image_type: Option<String>,
}
