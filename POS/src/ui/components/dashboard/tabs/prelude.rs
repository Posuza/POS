pub(crate) use crate::config::constants::{PRODUCTS_IMAGES_DIR, PROFILES_IMAGES_DIR};
pub(crate) use crate::config::settings::get_settings;
pub(crate) use crate::data::json_store::get_store_fresh;
pub(crate) use crate::data::models::product::Product;
pub(crate) use crate::data::models::user::User;
pub(crate) use crate::services::image_service::ImageService;
pub(crate) use crate::ui::components::{DashboardContainer, SalesTrend, TabContainer, Table};
pub(crate) use crate::utils::formatters::{format_datetime, format_price};
pub(crate) use base64::Engine;
pub(crate) use chrono::Utc;
pub(crate) use dioxus::prelude::*;
pub(crate) use serde::de::DeserializeOwned;
pub(crate) use serde_json::json;
pub(crate) use std::collections::HashMap;
pub(crate) use std::path::Path;
pub(crate) use std::time::Duration;
pub(crate) use uuid::Uuid;

pub(crate) const LOW_STOCK_THRESHOLD: i32 = 10;

pub(crate) async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::sleep(Duration::from_millis(ms)).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}

pub(crate) fn load_vec_from_json<T: DeserializeOwned>(key: &str) -> Vec<T> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join(key);
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

pub(crate) fn extra_list(key: &str) -> Vec<serde_json::Value> {
    load_vec_from_json(key)
}

pub(crate) fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() {
        return true;
    }
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let domain = parts[1];
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

pub(crate) fn load_image_from_path(path: &str) -> Result<(String, String, String), String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .ok_or_else(|| "Unsupported image type".to_string())?;
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let payload = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let data_url = format!("data:image/{};base64,{}", ext, payload);
    Ok((payload, ext, data_url))
}

pub(crate) fn infer_image_type_from_filename(name: &str) -> Option<String> {
    Path::new(name)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}

pub(crate) fn image_preview_from_filename(
    name: &str,
    image_type: &Option<String>,
    folder: &str,
) -> Option<String> {
    if name.starts_with("data:") {
        return Some(name.to_string());
    }
    let resolved = image_type
        .clone()
        .or_else(|| infer_image_type_from_filename(name));
    let Some(img_type) = resolved else {
        return None;
    };
    ImageService::get_image_data_url(name, &img_type, folder).ok()
}

pub(crate) fn value_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

pub(crate) fn value_f32(value: &serde_json::Value, key: &str) -> Option<f32> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_f64().map(|f| f as f32),
        serde_json::Value::String(s) => s.parse::<f32>().ok(),
        _ => None,
    })
}

pub(crate) fn value_i32(value: &serde_json::Value, key: &str) -> Option<i32> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_i64().map(|i| i as i32),
        serde_json::Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    })
}

pub(crate) fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

pub(crate) fn new_id(prefix: &str) -> String {
    format!("{}-{}", prefix, Uuid::new_v4().simple())
}

pub(crate) fn save_products_to_json(products: &[Product]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join("products.json");
    let contents = serde_json::to_string_pretty(products)
        .map_err(|e| format!("Serialize products failed: {}", e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write products failed: {}", e))?;
    Ok(())
}

pub(crate) fn save_sales_to_json(
    sales: &[crate::data::json_store::SaleRecord],
) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join("sales.json");
    let contents = serde_json::to_string_pretty(sales)
        .map_err(|e| format!("Serialize sales failed: {}", e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write sales failed: {}", e))?;
    Ok(())
}

pub(crate) fn export_payments_ledger(payments: &[serde_json::Value]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("payments_ledger.csv");
    let mut out = String::from("id,amount,method,status,created_at\n");
    for p in payments.iter() {
        let id = pick_first(p, &["id", "payment_id"]);
        let amount = pick_first(p, &["amount", "total"]);
        let method = pick_first(p, &["method", "type"]);
        let status = pick_first(p, &["status"]);
        let created_at = pick_first(p, &["created_at", "createdAt", "date"]);
        out.push_str(&format!(
            "{},{},{},{},{}\n",
            id, amount, method, status, created_at
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write ledger export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn export_payments_summary(payments: &[serde_json::Value]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("payments_summary.csv");
    let mut out = String::from("method,total\n");
    let mut totals: HashMap<String, f32> = HashMap::new();
    for p in payments.iter() {
        let method = pick_first(p, &["method", "type"]);
        let amount = value_f32(p, "amount")
            .or_else(|| value_f32(p, "total"))
            .unwrap_or(0.0);
        *totals.entry(method).or_insert(0.0) += amount;
    }
    for (method, total) in totals.into_iter() {
        out.push_str(&format!("{},{}\n", method, total));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write summary export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn export_products_csv(products: &[Product]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("products_export.csv");
    let mut out = String::from("id,barcode,name,price,quantity,category,created_at\n");
    for p in products.iter() {
        out.push_str(&format!(
            "{},{},{},{},{},{},{}\n",
            p.id, p.barcode, p.name, p.price, p.quantity, p.category, p.created_at
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write products export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn export_sales_csv(
    sales: &[crate::data::json_store::SaleRecord],
) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("sales_export.csv");
    let mut out = String::from("id,total,status,created_at\n");
    for s in sales.iter() {
        out.push_str(&format!(
            "{},{},{},{}\n",
            s.id, s.total, s.status, s.created_at
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write sales export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn save_payments_to_json(payments: &[serde_json::Value]) -> Result<(), String> {
    save_extra_to_json("payments.json", payments)
}

pub(crate) fn save_extra_to_json(key: &str, values: &[serde_json::Value]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join(key);
    let contents = serde_json::to_string_pretty(values)
        .map_err(|e| format!("Serialize {} failed: {}", key, e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write {} failed: {}", key, e))?;
    Ok(())
}

pub(crate) fn export_staff_csv(
    users: &[crate::data::json_store::UserRecord],
) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("staff_export.csv");
    let mut out = String::from("id,username,email,role,status\n");
    for u in users.iter() {
        out.push_str(&format!(
            "{},{},{},{:?},{}\n",
            u.id, u.username, u.email, u.role, u.status
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write staff export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn export_customers_csv(customers: &[serde_json::Value]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("customers_export.csv");
    let mut out = String::from("id,name,email,segment,status\n");
    for c in customers.iter() {
        let id = pick_first(c, &["id", "customer_id"]);
        let name = pick_first(c, &["name", "full_name", "username"]);
        let email = pick_first(c, &["email", "contact_email", "phone"]);
        let segment = pick_first(c, &["tier", "segment"]);
        let status = pick_first(c, &["status"]);
        out.push_str(&format!(
            "{},{},{},{},{}\n",
            id, name, email, segment, status
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write customers export failed: {}", e))?;
    Ok(path.display().to_string())
}

pub(crate) fn pick_first(value: &serde_json::Value, keys: &[&str]) -> String {
    for key in keys {
        if let Some(found) = value_string(value, key) {
            if !found.is_empty() {
                return found;
            }
        }
    }
    "—".to_string()
}

pub(crate) fn value_is(value: &serde_json::Value, key: &str, expected: &str) -> bool {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

pub(crate) fn normalize_sales_statuses(sales: &mut [crate::data::json_store::SaleRecord]) {
    for sale in sales.iter_mut() {
        let status = sale.status.trim().to_lowercase();
        let normalized = if status.contains("paid") || status.contains("completed") {
            "paid"
        } else if status.contains("void") || status.contains("cancel") {
            "voided"
        } else if status.contains("pending")
            || status.contains("due")
            || status.contains("processing")
        {
            "pending"
        } else {
            status.as_str()
        };
        sale.status = normalized.to_string();
    }
}

#[component]
pub(crate) fn StatCard(
    tone: &'static str,
    icon: &'static str,
    label: &'static str,
    value: String,
    trend: &'static str,
    trend_dir: &'static str,
    link: &'static str,
) -> Element {
    let card_class = format!("stat-card stat-card-{tone}");
    let icon_class = format!("stat-card-icon stat-card-icon-{tone}");
    let trend_class = format!("stat-trend {trend_dir}");

    rsx! {
        div { class: card_class,
            div { class: icon_class, "{icon}" }
            div { class: "stat-card-body",
                h4 { class: "stat-label", "{label}" }
                p { class: "stat-value", "{value}" }
                div { class: "stat-footer",
                    span { class: "{trend_class}", "{trend}" }
                    a { class: "stat-link", href: "{link}", "View report" }
                }
            }
        }
    }
}

pub(crate) fn status_chip_class(status: &str) -> &'static str {
    let normalized = status.trim().to_lowercase();
    if normalized.contains("active")
        || normalized.contains("paid")
        || normalized.contains("completed")
        || normalized.contains("received")
    {
        "status-chip"
    } else if normalized.contains("pending")
        || normalized.contains("due")
        || normalized.contains("processing")
    {
        "status-chip warning"
    } else if normalized.contains("inactive")
        || normalized.contains("cancel")
        || normalized.contains("expired")
        || normalized.contains("overdue")
    {
        "status-chip danger"
    } else {
        "status-chip"
    }
}

#[component]
pub(crate) fn StatusChip(label: String) -> Element {
    let class_name = format!("{}", status_chip_class(&label));
    rsx! {
        span { class: "{class_name}", "{label}" }
    }
}

#[component]
pub(crate) fn RowActions() -> Element {
    rsx! {
        div { class: "table-actions",
            button { class: "btn-secondary", "View" }
            button { class: "btn-secondary", "Edit" }
            button { class: "btn-danger", "Delete" }
        }
    }
}

#[component]
pub(crate) fn SearchInput(placeholder: &'static str) -> Element {
    rsx! {
        div { class: "input-group",
            span { class: "input-icon-left", "🔍" }
            input { r#type: "search", placeholder: "{placeholder}" }
        }
    }
}
