use base64::Engine;
use chrono::Local;
use std::fs;
use std::io::Read;
use std::path::Path;
use uuid::Uuid;

pub struct ImageService;

impl ImageService {
    /// Save product image (base64 payload) to products folder and return the stored filename.
    /// Images stored in: `data/images/products/`
    pub fn save_product_image(base64_data: &str, image_type: &str) -> Result<String, String> {
        let dir = crate::config::constants::PRODUCTS_IMAGES_DIR;
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S_%f").to_string();
        let filename = format!("product_{}_{}.{}", timestamp, Uuid::new_v4(), image_type);
        let file_path = format!("{}/{}", dir, filename);

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| e.to_string())?;

        fs::write(&file_path, decoded).map_err(|e| e.to_string())?;

        Ok(filename)
    }

    /// Save user profile image (base64 payload) to profiles folder and return the stored filename.
    /// Images stored in: `data/images/profiles/`
    pub fn save_user_image(
        base64_data: &str,
        image_type: &str,
        user_id: &str,
    ) -> Result<String, String> {
        let dir = crate::config::constants::PROFILES_IMAGES_DIR;
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;

        let timestamp = Local::now().format("%Y%m%d_%H%M%S_%f").to_string();
        // include user id for easier debugging/lookup
        let filename = format!(
            "user_{}_{}_{}.{}",
            user_id,
            timestamp,
            Uuid::new_v4(),
            image_type
        );
        let file_path = format!("{}/{}", dir, filename);

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(base64_data)
            .map_err(|e| e.to_string())?;

        fs::write(&file_path, decoded).map_err(|e| e.to_string())?;

        Ok(filename)
    }

    /// Load image from filename (in `folder`) and return as data URL `data:image/{type};base64,{payload}`
    pub fn get_image_data_url(
        filename: &str,
        image_type: &str,
        folder: &str,
    ) -> Result<String, String> {
        // If `filename` already looks like a data URL, return it directly
        if filename.starts_with("data:") {
            return Ok(filename.to_string());
        }

        let path = format!("{}/{}", folder, filename);
        let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&buf);
        Ok(format!("data:image/{};base64,{}", image_type, encoded))
    }

    /// Get the filesystem path for an image kept in `folder`.
    pub fn get_image_path(filename: &str, folder: &str) -> String {
        format!("{}/{}", folder, filename)
    }

    /// Delete an image file in `folder`.
    pub fn delete_image(filename: &str, folder: &str) -> Result<(), String> {
        let path = format!("{}/{}", folder, filename);
        fs::remove_file(&path).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Check whether an image file exists in `folder`.
    pub fn image_exists(filename: &str, folder: &str) -> bool {
        let path = format!("{}/{}", folder, filename);
        Path::new(&path).exists()
    }
}
