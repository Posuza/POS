-- Users table WITH PROFILE IMAGE
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('admin', 'staff')),
    status TEXT DEFAULT 'active',
    profile_image TEXT,              -- 📸 Image path or base64
    profile_image_type TEXT,         -- jpeg, png, etc
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Products table WITH PRODUCT IMAGE
CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    barcode TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    price REAL NOT NULL,
    quantity INTEGER DEFAULT 0,
    category TEXT,
    product_image TEXT,              -- 📸 Image path or base64
    product_image_type TEXT,         -- jpeg, png, etc
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Scans table (Transaction history)
CREATE TABLE IF NOT EXISTS scans (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL,
    staff_id TEXT NOT NULL,
    barcode TEXT NOT NULL,
    quantity INTEGER DEFAULT 1,
    price REAL,
    scanned_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id),
    FOREIGN KEY (staff_id) REFERENCES users(id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_barcode ON products(barcode);
CREATE INDEX IF NOT EXISTS idx_staff_id ON scans(staff_id);
CREATE INDEX IF NOT EXISTS idx_scanned_at ON scans(scanned_at);

-- Demo data WITH IMAGES
INSERT OR IGNORE INTO users (id, username, email, password_hash, role) VALUES
('admin1', 'admin', 'admin@pos.local', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5YmMxSUmGEJiq', 'admin'),
('staff1', 'staff', 'staff@pos.local', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5YmMxSUmGEJiq', 'staff');

INSERT OR IGNORE INTO products (id, barcode, name, category, price, quantity) VALUES
('prod1', '123456789012', 'Apple', 'Fruits', 1.50, 100),
('prod2', '123456789013', 'Banana', 'Fruits', 0.99, 150),
('prod3', '123456789014', 'Milk', 'Dairy', 2.99, 50),
('prod4', '123456789015', 'Bread', 'Bakery', 3.49, 30);