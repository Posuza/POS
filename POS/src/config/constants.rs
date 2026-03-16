/// Global constants for POS application

// Database
pub const DATABASE_FILE: &str = "data/pos_data.db";
pub const DATABASE_URL: &str = "sqlite://data/pos_data.db";

// JSON data
pub const JSON_DATA_DIR: &str = "data/json";

// Image storage
pub const IMAGES_DIR: &str = "data/images";
pub const PRODUCTS_IMAGES_DIR: &str = "data/images/products";
pub const PROFILES_IMAGES_DIR: &str = "data/images/profiles";
pub const MAX_IMAGE_SIZE: usize = 5 * 1024 * 1024; // 5MB
pub const SUPPORTED_IMAGE_FORMATS: &[&str] = &["jpeg", "jpg", "png", "gif", "webp"];

// Authentication
pub const PASSWORD_MIN_LENGTH: usize = 8;
pub const PASSWORD_MAX_LENGTH: usize = 128;
pub const BCRYPT_COST: u32 = 12;
pub const JWT_EXPIRY_HOURS: i64 = 24;

// UI
pub const APP_NAME: &str = "POS System";
pub const APP_VERSION: &str = "1.0.0";

// Validation
pub const BARCODE_MIN_LENGTH: usize = 8;
pub const BARCODE_MAX_LENGTH: usize = 20;
pub const USERNAME_MIN_LENGTH: usize = 3;
pub const USERNAME_MAX_LENGTH: usize = 32;
pub const EMAIL_MAX_LENGTH: usize = 100;

// Payment methods
pub const PAYMENT_METHODS: &[&str] = &["Cash", "Card", "Check", "Mobile"];

// Demo credentials
pub const DEMO_ADMIN_USERNAME: &str = "admin";
pub const DEMO_ADMIN_PASSWORD: &str = "admin123";
pub const DEMO_STAFF_USERNAME: &str = "staff";
pub const DEMO_STAFF_PASSWORD: &str = "staff123";
