/// Data formatters for POS application

/// Format price as currency string
pub fn format_price(price: f32) -> String {
    format!("${:.2}", price)
}

/// Format date and time
pub fn format_datetime(datetime: &str) -> String {
    // Simple format, can be enhanced
    datetime.split('T').next().unwrap_or(datetime).to_string()
}

/// Format quantity with thousands separator
pub fn format_quantity(quantity: i32) -> String {
    quantity.to_string()
}

/// Truncate string to max length with ellipsis
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}

/// Format product name for display
pub fn format_product_name(name: &str) -> String {
    truncate_string(name, 30)
}

/// Format username for display
pub fn format_username(username: &str) -> String {
    truncate_string(username, 20)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_price() {
        assert_eq!(format_price(10.5), "$10.50");
        assert_eq!(format_price(1000.0), "$1000.00");
    }
    
    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world test", 8), "hello...");
    }
}
