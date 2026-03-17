/// Input validators for POS application
use crate::config::constants::*;
use crate::errors::{PosError, Result};

/// Validate username
pub fn validate_username(username: &str) -> Result<()> {
    if username.is_empty() {
        return Err(PosError::ValidationError(
            "Username cannot be empty".to_string(),
        ));
    }

    if username.len() < USERNAME_MIN_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Username must be at least {} characters",
            USERNAME_MIN_LENGTH
        )));
    }

    if username.len() > USERNAME_MAX_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Username must not exceed {} characters",
            USERNAME_MAX_LENGTH
        )));
    }

    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(PosError::ValidationError(
            "Username can only contain alphanumeric characters, underscore, and hyphen".to_string(),
        ));
    }

    Ok(())
}

/// Validate email
pub fn validate_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(PosError::ValidationError(
            "Email cannot be empty".to_string(),
        ));
    }

    if email.len() > EMAIL_MAX_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Email must not exceed {} characters",
            EMAIL_MAX_LENGTH
        )));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(PosError::ValidationError(
            "Invalid email format".to_string(),
        ));
    }

    Ok(())
}

/// Validate password strength
pub fn validate_password(password: &str) -> Result<()> {
    if password.is_empty() {
        return Err(PosError::ValidationError(
            "Password cannot be empty".to_string(),
        ));
    }

    if password.len() < PASSWORD_MIN_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Password must be at least {} characters",
            PASSWORD_MIN_LENGTH
        )));
    }

    if password.len() > PASSWORD_MAX_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Password must not exceed {} characters",
            PASSWORD_MAX_LENGTH
        )));
    }

    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());

    if !has_uppercase || !has_lowercase || !has_digit {
        return Err(PosError::ValidationError(
            "Password must contain uppercase, lowercase, and numeric characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate barcode
pub fn validate_barcode(barcode: &str) -> Result<()> {
    if barcode.is_empty() {
        return Err(PosError::ValidationError(
            "Barcode cannot be empty".to_string(),
        ));
    }

    if barcode.len() < BARCODE_MIN_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Barcode must be at least {} characters",
            BARCODE_MIN_LENGTH
        )));
    }

    if barcode.len() > BARCODE_MAX_LENGTH {
        return Err(PosError::ValidationError(format!(
            "Barcode must not exceed {} characters",
            BARCODE_MAX_LENGTH
        )));
    }

    if !barcode.chars().all(|c| c.is_numeric() || c.is_alphabetic()) {
        return Err(PosError::ValidationError(
            "Barcode must contain only alphanumeric characters".to_string(),
        ));
    }

    Ok(())
}

/// Validate product price
pub fn validate_price(price: f32) -> Result<()> {
    if price < 0.0 {
        return Err(PosError::ValidationError(
            "Price cannot be negative".to_string(),
        ));
    }

    if price > 1_000_000.0 {
        return Err(PosError::ValidationError("Price is too high".to_string()));
    }

    Ok(())
}

/// Validate product quantity
pub fn validate_quantity(quantity: i32) -> Result<()> {
    if quantity < 0 {
        return Err(PosError::ValidationError(
            "Quantity cannot be negative".to_string(),
        ));
    }

    if quantity > 1_000_000 {
        return Err(PosError::ValidationError(
            "Quantity is too high".to_string(),
        ));
    }

    Ok(())
}

/// Validate image format
pub fn validate_image_format(format: &str) -> Result<()> {
    if SUPPORTED_IMAGE_FORMATS.contains(&format) {
        Ok(())
    } else {
        Err(PosError::ValidationError(format!(
            "Unsupported image format: {}. Supported: {:?}",
            format, SUPPORTED_IMAGE_FORMATS
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("user_123").is_ok());
        assert!(validate_username("").is_err());
        assert!(validate_username("ab").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("SecurePass123").is_ok());
        assert!(validate_password("weak").is_err());
        assert!(validate_password("NoDigit123").is_ok());
    }

    #[test]
    fn test_validate_barcode() {
        assert!(validate_barcode("123456789012").is_ok());
        assert!(validate_barcode("1234567890123456789012").is_err());
        assert!(validate_barcode("1234").is_err());
    }
}
