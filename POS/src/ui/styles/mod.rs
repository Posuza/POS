/// Styles module — returns all bundled CSS as a compile-time static string.
/// CSS files are collected and concatenated by build.rs at compile time.
pub fn get_styles() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/styles.css"))
}
