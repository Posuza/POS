/// Styles module

/// Return combined styles as a compile-time static string to avoid
/// allocating/joining on every call (helps rendering performance).
pub fn get_styles() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/styles.css"))
}
