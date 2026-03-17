use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub status: String,
    pub profile_image: Option<String>,      // 📸 Base64 or path
    pub profile_image_type: Option<String>, // jpeg, png
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UserRole {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "staff")]
    Staff,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Staff => "staff",
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct AuthState {
    pub user: Option<User>,
    pub is_authenticated: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        AuthState {
            user: None,
            is_authenticated: false,
        }
    }
}
