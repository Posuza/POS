/// User queries module

use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Local;
use bcrypt::{hash, verify};

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct UserDB {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub status: String,
    pub profile_image: Option<String>,        // 📸 FILENAME ONLY (stored in data/images/profiles/)
    pub profile_image_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_user(
    db: &SqlitePool,
    username: &str,
    email: &str,
    password: &str,
    role: &str,
) -> Result<UserDB, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let password_hash = hash(password, 12).map_err(|e| {
        sqlx::Error::Configuration(e.to_string().into())
    })?;
    let now = Local::now().to_rfc3339();
    
    sqlx::query_as::<_, UserDB>(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(id)
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .bind(&now)
    .bind(&now)
    .fetch_one(db)
    .await
}

/// Update user profile image - stores filename only
pub async fn update_user_image(
    db: &SqlitePool,
    user_id: &str,
    image_filename: &str,  // Just filename, not base64
    image_type: &str,
) -> Result<(), sqlx::Error> {
    let now = Local::now().to_rfc3339();
    
    sqlx::query(
        "UPDATE users SET profile_image = ?, profile_image_type = ?, updated_at = ? WHERE id = ?"
    )
    .bind(image_filename)      // Store only filename
    .bind(image_type)
    .bind(now)
    .bind(user_id)
    .execute(db)
    .await?;
    
    Ok(())
}

pub async fn get_user_by_username(
    db: &SqlitePool,
    username: &str,
) -> Result<Option<UserDB>, sqlx::Error> {
    sqlx::query_as::<_, UserDB>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(db)
    .await
}

pub async fn verify_password(
    db: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<bool, String> {
    match get_user_by_username(db, username).await {
        Ok(Some(user)) => {
            verify(password, &user.password_hash)
                .map_err(|e| e.to_string())
        }
        Ok(None) => Ok(false),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_all_users(db: &SqlitePool) -> Result<Vec<UserDB>, sqlx::Error> {
    sqlx::query_as::<_, UserDB>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(db)
    .await
}
