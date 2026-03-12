/// Helper functions for POS application

use std::fs;
use crate::errors::Result;

/// Ensure directories exist
pub fn ensure_directories() -> Result<()> {
    let dirs = vec![
        "data",
        "data/images",
        "data/images/products",
        "data/images/profiles",
    ];
    
    for dir in dirs {
        fs::create_dir_all(dir)
            .map_err(|e| crate::errors::PosError::FileError(format!(
                "Failed to create directory {}: {}",
                dir, e
            )))?;
    }
    
    Ok(())
}

/// Generate unique filename with timestamp
pub fn generate_filename(prefix: &str, extension: &str) -> String {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S_%f");
    let uuid = uuid::Uuid::new_v4().to_string()[..8].to_string();
    format!("{}_{}_{}. {}", prefix, timestamp, uuid, extension)
}

/// Get file size in bytes
pub fn get_file_size(path: &str) -> Result<u64> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| crate::errors::PosError::FileError(e.to_string()))
}

/// Format bytes as human readable
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    
    for (i, unit) in UNITS.iter().enumerate() {
        if size < 1024.0 || i == UNITS.len() - 1 {
            if i == 0 {
                return format!("{:.0} {}", size, unit);
            } else {
                return format!("{:.2} {}", size, unit);
            }
        }
        size /= 1024.0;
    }
    
    format!("{:.2} B", bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert!(format_bytes(1024).contains("KB"));
        assert!(format_bytes(1024 * 1024).contains("MB"));
    }
    
    #[test]
    fn test_generate_filename() {
        let filename = generate_filename("product", "jpg");
        assert!(filename.contains("product"));
        assert!(filename.contains("jpg"));
    }
}
