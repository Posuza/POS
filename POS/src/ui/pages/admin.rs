use dioxus::prelude::*;
use crate::data::models::user::User;
use crate::data::models::product::Product;
use crate::data::json_store::get_store_fresh;
use crate::utils::formatters::{format_price, format_datetime};
use crate::ui::components::{Navbar, SalesTrend};
use crate::config::constants::APP_NAME;
use crate::config::constants::{PRODUCTS_IMAGES_DIR, PROFILES_IMAGES_DIR};
use crate::config::settings::get_settings;
use crate::services::image_service::ImageService;
use std::time::Duration;
use std::collections::HashMap;
use std::path::Path;
use serde_json::json;
use serde::de::DeserializeOwned;
use base64::Engine;
use chrono::Utc;
use uuid::Uuid;


async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::sleep(Duration::from_millis(ms)).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    }
}

#[component]
pub fn AdminPage(user: User, on_logout: EventHandler<()>) -> Element {
    let mut active_tab = use_signal(|| AdminTab::Dashboard);
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        div {
            Navbar { 
                title: APP_NAME.to_string(),
                user: Some(user.clone()),
                on_login: move |_| (),
                on_logout: on_logout.clone(),
            }

            // Floating Sidebar Menu
            SidebarMenu {
                active_tab: active_tab.clone(),
                on_select: move |tab: AdminTab| active_tab.set(tab),
                is_open: sidebar_open.clone(),
                on_close: move |_| sidebar_open.set(false),
            }

            div { class: "admin-page",
                div { class: "admin-container",
                    // Menu toggle button (visible on all screens, especially mobile)
                    div {
                        class: "admin-menu-toggle-wrap",
                        button {
                            class: "admin-menu-toggle",
                            aria_label: "Open admin menu",
                            onclick: move |_| {
                                let next_state = !*sidebar_open.read();
                                sidebar_open.set(next_state);
                            },
                            span { class: "admin-menu-toggle-icon", "☰" }
                            span { class: "admin-menu-toggle-label", "Menu" }
                        }
                    }

                    // Content
                    div { class: "admin-main",
                        {
                    let (tab_label, tab_subtitle) = tab_metadata(*active_tab.read());
                    let store = get_store_fresh();
                    let last_updated = store.sales
                        .last()
                        .map(|s| format_datetime(&s.created_at))
                        .unwrap_or_else(|| "No recent updates".to_string());
                    rsx!(
                        div { class: "admin-page-header",
                            div { class: "breadcrumb",
                                span { "Admin" }
                                span { class: "breadcrumb-sep", "›" }
                                span { class: "breadcrumb-current", "{tab_label}" }
                            }
                            h1 { class: "page-title", "{tab_label}" }
                            p { class: "page-subtitle", "{tab_subtitle}" }
                            span { class: "page-updated", "Last updated: {last_updated}" }
                        }
                    )
                }
                        match *active_tab.read() {
                            AdminTab::Dashboard => rsx! { DashboardTab { user: user.clone() } },
                            AdminTab::SuperAdmin => rsx! { SuperAdminTab {} },
                            AdminTab::SalesDashboard => rsx! { SalesDashboardTab {} },
                            AdminTab::Sales => rsx! { SalesTab {} },
                            AdminTab::Products => rsx! { ProductsTab {} },
                            AdminTab::CreateProduct => rsx! { CreateProductTab {} },
                            AdminTab::ExpiredProducts => rsx! { ExpiredProductsTab {} },
                            AdminTab::LowStocks => rsx! { LowStocksTab {} },
                            AdminTab::Categories => rsx! { CategoriesTab {} },
                            AdminTab::SubCategories => rsx! { SubCategoriesTab {} },
                            AdminTab::Brands => rsx! { BrandsTab {} },
                            AdminTab::Units => rsx! { UnitsTab {} },
                            AdminTab::VariantAttributes => rsx! { VariantAttributesTab {} },
                            AdminTab::Warranties => rsx! { WarrantiesTab {} },
                            AdminTab::PrintBarcode => rsx! { PrintBarcodeTab {} },
                            AdminTab::PrintQrCode => rsx! { PrintQrCodeTab {} },
                            AdminTab::Warehouses => rsx! { WarehousesTab {} },
                            AdminTab::Stores => rsx! { StoresTab {} },
                            AdminTab::Billers => rsx! { BillersTab {} },
                            AdminTab::ManageStock => rsx! { ManageStockTab {} },
                            AdminTab::StockAdjustment => rsx! { StockAdjustmentTab {} },
                            AdminTab::StockTransfer => rsx! { StockTransferTab {} },
                            AdminTab::InvoiceReport => rsx! { InvoiceReportTab {} },
                            AdminTab::SupplierReport => rsx! { SupplierReportTab {} },
                            AdminTab::CustomerReport => rsx! { CustomerReportTab {} },
                            AdminTab::ProductReport => rsx! { ProductReportTab {} },
                            AdminTab::InventoryReport => rsx! { InventoryReportTab {} },
                            AdminTab::PurchaseReport => rsx! { PurchaseReportTab {} },
                            AdminTab::SalesReport => rsx! { SalesReportTab {} },
                            AdminTab::Staff => rsx! { StaffTab {} },
                            AdminTab::Customers => rsx! { CustomersTab {} },
                            AdminTab::Inventory => rsx! { InventoryTab {} },
                            AdminTab::Payments => rsx! { PaymentsTab {} },
                            AdminTab::Access => rsx! { AccessTab {} },
                            AdminTab::Suppliers => rsx! { SuppliersTab {} },
                            AdminTab::Shifts => rsx! { ShiftsTab {} },
                            AdminTab::Settings => rsx! { SettingsTab {} },
                            AdminTab::Placeholder(title) => rsx! { PlaceholderTab { title: title } },
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AdminTab {
    Dashboard,
    SuperAdmin,
    SalesDashboard,
    Sales,
    Products,
    CreateProduct,
    ExpiredProducts,
    LowStocks,
    Categories,
    SubCategories,
    Brands,
    Units,
    VariantAttributes,
    Warranties,
    PrintBarcode,
    PrintQrCode,
    Warehouses,
    Stores,
    Billers,
    ManageStock,
    StockAdjustment,
    StockTransfer,
    InvoiceReport,
    SupplierReport,
    CustomerReport,
    ProductReport,
    InventoryReport,
    PurchaseReport,
    SalesReport,
    Staff,
    Customers,
    Inventory,
    Payments,
    Access,
    Suppliers,
    Shifts,
    Settings,
    Placeholder(&'static str),
}

#[component]
fn SidebarMenu(active_tab: Signal<AdminTab>, on_select: EventHandler<AdminTab>, is_open: Signal<bool>, on_close: EventHandler<()>) -> Element {
    let drawer_open = *is_open.read();
    let current_tab = *active_tab.read();
    let menu_groups: Vec<(&'static str, Vec<(AdminTab, &'static str, &'static str)>)> = vec![
        ("Main", vec![
            (AdminTab::Dashboard, "Dashboard", "🏠"),
            (AdminTab::SuperAdmin, "Super Admin", "🧭"),
        ]),
        ("Inventory", vec![
            (AdminTab::Products, "Products", "📦"),
            (AdminTab::CreateProduct, "Create Product", "➕"),
            (AdminTab::ExpiredProducts, "Expired Products", "⏳"),
            (AdminTab::LowStocks, "Low Stocks", "⚠️"),
            (AdminTab::Categories, "Category", "🗂️"),
            (AdminTab::SubCategories, "Sub Category", "🧩"),
            (AdminTab::Brands, "Brands", "🏷️"),
            (AdminTab::Units, "Units", "📏"),
            (AdminTab::VariantAttributes, "Variant Attributes", "🧬"),
            (AdminTab::Warranties, "Warranties", "🛡️"),
            (AdminTab::PrintBarcode, "Print Barcode", "🏷️"),
            (AdminTab::PrintQrCode, "Print QR Code", "🔳"),
        ]),
        ("Stock", vec![
            (AdminTab::ManageStock, "Manage Stock", "📦"),
            (AdminTab::StockAdjustment, "Stock Adjustment", "🛠️"),
            (AdminTab::StockTransfer, "Stock Transfer", "🔁"),
        ]),
        ("Sales", vec![
            (AdminTab::SalesDashboard, "Sales Dashboard", "📊"),
            (AdminTab::Sales, "Sales", "💸"),
            (AdminTab::Placeholder("Invoices"), "Invoices", "🧾"),
            (AdminTab::Placeholder("Sales Return"), "Sales Return", "↩️"),
            (AdminTab::Placeholder("Quotation"), "Quotation", "📝"),
            (AdminTab::Placeholder("POS"), "POS", "🛒"),
        ]),
        ("Promo", vec![
            (AdminTab::Placeholder("Coupons"), "Coupons", "🏷️"),
            (AdminTab::Placeholder("Gift Card"), "Gift Card", "🎁"),
            (AdminTab::Placeholder("Discount"), "Discount", "💯"),
        ]),
        ("Purchases", vec![
            (AdminTab::Placeholder("Purchases"), "Purchases", "🧺"),
            (AdminTab::Placeholder("Purchase Order"), "Purchase Order", "📋"),
            (AdminTab::Placeholder("Purchase Return"), "Purchase Return", "↩️"),
        ]),
        ("Finance & Accounts", vec![
            (AdminTab::Placeholder("Expenses"), "Expenses", "💼"),
            (AdminTab::Placeholder("Income"), "Income", "💰"),
            (AdminTab::Placeholder("Bank Accounts"), "Bank Accounts", "🏦"),
            (AdminTab::Placeholder("Money Transfer"), "Money Transfer", "💸"),
            (AdminTab::Placeholder("Balance Sheet"), "Balance Sheet", "📄"),
            (AdminTab::Placeholder("Trial Balance"), "Trial Balance", "🧮"),
            (AdminTab::Placeholder("Cash Flow"), "Cash Flow", "🌊"),
            (AdminTab::Placeholder("Account Statement"), "Account Statement", "📃"),
        ]),
        ("People", vec![
            (AdminTab::Customers, "Customers", "🧑‍🤝‍🧑"),
            (AdminTab::Billers, "Billers", "🧾"),
            (AdminTab::Suppliers, "Suppliers", "🚚"),
            (AdminTab::Stores, "Stores", "🏪"),
            (AdminTab::Warehouses, "Warehouses", "🏭"),
        ]),
        ("HRM", vec![
            (AdminTab::Placeholder("Employees"), "Employees", "👥"),
            (AdminTab::Placeholder("Departments"), "Departments", "🏢"),
            (AdminTab::Placeholder("Designation"), "Designation", "🎯"),
            (AdminTab::Shifts, "Shifts", "🕒"),
            (AdminTab::Placeholder("Attendance"), "Attendance", "📌"),
            (AdminTab::Placeholder("Leaves"), "Leaves", "🌴"),
            (AdminTab::Placeholder("Holidays"), "Holidays", "🎉"),
            (AdminTab::Placeholder("Payroll"), "Payroll", "💵"),
        ]),
        ("Reports", vec![
            (AdminTab::SalesReport, "Sales Report", "📈"),
            (AdminTab::PurchaseReport, "Purchase Report", "🧾"),
            (AdminTab::InventoryReport, "Inventory Report", "📦"),
            (AdminTab::InvoiceReport, "Invoice Report", "🧾"),
            (AdminTab::SupplierReport, "Supplier Report", "🚚"),
            (AdminTab::CustomerReport, "Customer Report", "🧑‍🤝‍🧑"),
            (AdminTab::ProductReport, "Product Report", "📊"),
            (AdminTab::Placeholder("Expense Report"), "Expense Report", "💼"),
            (AdminTab::Placeholder("Income Report"), "Income Report", "💰"),
            (AdminTab::Placeholder("Tax Report"), "Tax Report", "🧾"),
            (AdminTab::Placeholder("Profit & Loss"), "Profit & Loss", "📈"),
            (AdminTab::Placeholder("Annual Report"), "Annual Report", "📅"),
        ]),
        ("User Management", vec![
            (AdminTab::Placeholder("Users"), "Users", "👤"),
            (AdminTab::Access, "Roles & Permissions", "🛡️"),
            (AdminTab::Placeholder("Delete Account Request"), "Delete Account Request", "🗑️"),
        ]),
        ("Content (CMS)", vec![
            (AdminTab::Placeholder("Pages"), "Pages", "📄"),
            (AdminTab::Placeholder("Blog"), "Blog", "📰"),
            (AdminTab::Placeholder("Location"), "Location", "📍"),
            (AdminTab::Placeholder("Testimonials"), "Testimonials", "💬"),
            (AdminTab::Placeholder("FAQ"), "FAQ", "❓"),
        ]),
        ("Pages", vec![
            (AdminTab::Placeholder("Profile"), "Profile", "👤"),
            (AdminTab::Placeholder("Authentication"), "Authentication", "🔐"),
            (AdminTab::Placeholder("Error Page"), "Error Page", "⚠️"),
            (AdminTab::Placeholder("Blank Page"), "Blank Page", "⬜"),
            (AdminTab::Placeholder("Pricing"), "Pricing", "💲"),
            (AdminTab::Placeholder("Coming Soon"), "Coming Soon", "⏳"),
            (AdminTab::Placeholder("Under Maintenance"), "Under Maintenance", "🛠️"),
        ]),
        ("Settings", vec![
            (AdminTab::Settings, "General Settings", "⚙️"),
            (AdminTab::Placeholder("Website Settings"), "Website Settings", "🌐"),
            (AdminTab::Placeholder("App Settings"), "App Settings", "📱"),
            (AdminTab::Placeholder("System Settings"), "System Settings", "🖥️"),
            (AdminTab::Placeholder("Financial Settings"), "Financial Settings", "💵"),
            (AdminTab::Placeholder("Other Settings"), "Other Settings", "🔧"),
            (AdminTab::Placeholder("Logout"), "Logout", "🚪"),
        ]),
    ];

    rsx! {
        if drawer_open {
            // Overlay backdrop (closes sidebar on click)
            div {
                class: "admin-sidebar-overlay",
                onclick: move |_| on_close.call(()),
            }
        }

        aside {
            class: if drawer_open { "admin-sidebar-drawer open" } else { "admin-sidebar-drawer" },
            
            div {
                class: "admin-sidebar-header",
                h3 { "DreamsPOS" }
                button {
                    class: "admin-sidebar-close",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            nav {
                class: "admin-sidebar-nav",
                for (group_label, items) in menu_groups {
                    div { class: "admin-sidebar-group",
                        h4 { class: "admin-sidebar-group-title", "{group_label}" }
                        for (tab, label, icon) in items {
                            button {
                                class: if current_tab == tab { "admin-sidebar-item active" } else { "admin-sidebar-item" },
                                aria_label: "{label}",
                                onclick: move |_| {
                                    on_select.call(tab);
                                    on_close.call(());
                                },
                                span { class: "admin-sidebar-icon", "{icon}" }
                                span { class: "admin-sidebar-label", "{label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn tab_metadata(tab: AdminTab) -> (&'static str, &'static str) {
    match tab {
        AdminTab::Dashboard => ("Dashboard", "Live operational overview and insights."),
        AdminTab::SuperAdmin => ("Super Admin", "System-wide access and master controls."),
        AdminTab::SalesDashboard => ("Sales Dashboard", "Sales KPIs, trends, and recent performance."),
        AdminTab::Sales => ("Sales", "Review transactions and revenue performance."),
        AdminTab::SalesReport => ("Sales Report", "Summary of sales and performance trends."),
        AdminTab::Products => ("Products", "Manage inventory and product catalog."),
        AdminTab::CreateProduct => ("Create Product", "Add a new item to your catalog."),
        AdminTab::ExpiredProducts => ("Expired Products", "Track expired or near-expiry inventory."),
        AdminTab::LowStocks => ("Low Stocks", "Monitor products below reorder levels."),
        AdminTab::Categories => ("Category", "Organize product categories."),
        AdminTab::SubCategories => ("Sub Category", "Organize product subcategories."),
        AdminTab::Brands => ("Brands", "Manage product brands."),
        AdminTab::Units => ("Units", "Configure measurement units."),
        AdminTab::VariantAttributes => ("Variant Attributes", "Define product variants and options."),
        AdminTab::Warranties => ("Warranties", "Manage warranty coverage details."),
        AdminTab::PrintBarcode => ("Print Barcode", "Generate and print product barcodes."),
        AdminTab::PrintQrCode => ("Print QR Code", "Generate and print product QR codes."),
        AdminTab::Warehouses => ("Warehouses", "Manage warehouse locations and stock."),
        AdminTab::Stores => ("Stores", "Manage store locations and contacts."),
        AdminTab::Billers => ("Billers", "Manage billing contacts and partners."),
        AdminTab::ManageStock => ("Manage Stock", "Track stock adjustments and assignments."),
        AdminTab::StockAdjustment => ("Stock Adjustment", "Record stock corrections and audits."),
        AdminTab::StockTransfer => ("Stock Transfer", "Move inventory between locations."),
        AdminTab::InvoiceReport => ("Invoice Report", "Invoice status and collections overview."),
        AdminTab::SupplierReport => ("Supplier Report", "Supplier activity and balances."),
        AdminTab::CustomerReport => ("Customer Report", "Customer orders and payment behavior."),
        AdminTab::ProductReport => ("Product Report", "Top products and inventory revenue."),
        AdminTab::InventoryReport => ("Inventory Report", "Inventory summary and stock snapshots."),
        AdminTab::PurchaseReport => ("Purchase Report", "Purchases and supplier spend details."),
        AdminTab::Staff => ("Staff", "Control staff access and schedules."),
        AdminTab::Customers => ("Customers", "Track customer profiles and engagement."),
        AdminTab::Inventory => ("Inventory", "Monitor stock movements and scan activity."),
        AdminTab::Payments => ("Payments", "Review payment records and settlement status."),
        AdminTab::Access => ("Access Control", "Manage roles, permissions, and assignments."),
        AdminTab::Suppliers => ("Suppliers", "Coordinate supplier contacts and performance."),
        AdminTab::Shifts => ("Shifts", "Coordinate staffing coverage and schedules."),
        AdminTab::Settings => ("Settings", "Configure store preferences and system settings."),
        AdminTab::Placeholder(label) => (label, "Coming soon."),
    }
}

// `SidebarMenu` renders the floating admin drawer entries directly.

fn load_vec_from_json<T: DeserializeOwned>(key: &str) -> Vec<T> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join(key);
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

fn extra_list(key: &str) -> Vec<serde_json::Value> {
    load_vec_from_json(key)
}

fn is_valid_email(email: &str) -> bool {
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

fn load_image_from_path(path: &str) -> Result<(String, String, String), String> {
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

fn infer_image_type_from_filename(name: &str) -> Option<String> {
    Path::new(name)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}

fn image_preview_from_filename(name: &str, image_type: &Option<String>, folder: &str) -> Option<String> {
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

fn value_string(value: &serde_json::Value, key: &str) -> Option<String> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    })
}

fn value_f32(value: &serde_json::Value, key: &str) -> Option<f32> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_f64().map(|f| f as f32),
        serde_json::Value::String(s) => s.parse::<f32>().ok(),
        _ => None,
    })
}

fn value_i32(value: &serde_json::Value, key: &str) -> Option<i32> {
    value.get(key).and_then(|v| match v {
        serde_json::Value::Number(n) => n.as_i64().map(|i| i as i32),
        serde_json::Value::String(s) => s.parse::<i32>().ok(),
        _ => None,
    })
}

fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

fn new_id(prefix: &str) -> String {
    format!("{}-{}", prefix, Uuid::new_v4().simple())
}

const LOW_STOCK_THRESHOLD: i32 = 60;

fn save_products_to_json(products: &[Product]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join("products.json");
    let contents = serde_json::to_string_pretty(products)
        .map_err(|e| format!("Serialize products failed: {}", e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write products.json failed: {}", e))?;
    Ok(())
}

fn save_sales_to_json(sales: &[crate::data::json_store::SaleRecord]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join("sales.json");
    let contents = serde_json::to_string_pretty(sales)
        .map_err(|e| format!("Serialize sales failed: {}", e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write sales.json failed: {}", e))?;
    Ok(())
}

fn export_payments_ledger(payments: &[serde_json::Value]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("payment_ledger.csv");
    let mut out = String::from("id,sale_id,method,amount,currency,status,reference,paid_at\n");
    for p in payments.iter() {
        let id = pick_first(p, &["id"]);
        let sale_id = pick_first(p, &["sale_id", "saleId"]);
        let method = pick_first(p, &["method", "type", "channel"]);
        let amount = value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0);
        let currency = pick_first(p, &["currency"]);
        let status = pick_first(p, &["status", "state"]);
        let reference = pick_first(p, &["reference", "ref"]);
        let paid_at = pick_first(p, &["paid_at", "created_at", "date"]);
        out.push_str(&format!("{},{},{},{:.2},{},{},{},{}\n", id, sale_id, method, amount, currency, status, reference, paid_at));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write ledger failed: {}", e))?;
    Ok(path.display().to_string())
}

fn export_payments_summary(payments: &[serde_json::Value]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("settlement_summary.csv");
    let mut method_totals: HashMap<String, (i32, f32)> = HashMap::new();
    for p in payments.iter() {
        let method = pick_first(p, &["method", "type", "channel"]);
        let amount = value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0);
        let entry = method_totals.entry(method).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += amount;
    }
    let mut out = String::from("method,count,total\n");
    for (method, (count, total)) in method_totals.into_iter() {
        out.push_str(&format!("{},{},{:.2}\n", method, count, total));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write summary failed: {}", e))?;
    Ok(path.display().to_string())
}

fn export_products_csv(products: &[Product]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("products_export.csv");
    let mut out = String::from("id,barcode,name,category,price,quantity\n");
    for p in products.iter() {
        out.push_str(&format!("{},{},{},{},{:.2},{}\n", p.id, p.barcode, p.name, p.category, p.price, p.quantity));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write products export failed: {}", e))?;
    Ok(path.display().to_string())
}

fn export_sales_csv(sales: &[crate::data::json_store::SaleRecord]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("sales_export.csv");
    let mut out = String::from("id,receipt_no,cashier_id,customer_id,subtotal,tax,discount,total,status,created_at\n");
    for s in sales.iter() {
        let customer = s.customer_id.clone().unwrap_or_default();
        out.push_str(&format!(
            "{},{},{},{},{:.2},{:.2},{:.2},{:.2},{},{}\n",
            s.id, s.receipt_no, s.cashier_id, customer, s.subtotal, s.tax, s.discount, s.total, s.status, s.created_at
        ));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write sales export failed: {}", e))?;
    Ok(path.display().to_string())
}

fn normalize_sales_statuses(sales: &mut [crate::data::json_store::SaleRecord]) {
    for s in sales.iter_mut() {
        let status = s.status.to_lowercase();
        if status == "completed" {
            s.status = "paid".to_string();
        } else if status == "void" {
            s.status = "voided".to_string();
        } else if status.is_empty() {
            s.status = "pending".to_string();
        }
    }
}

fn save_payments_to_json(payments: &[serde_json::Value]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join("payments.json");
    let contents = serde_json::to_string_pretty(payments)
        .map_err(|e| format!("Serialize payments failed: {}", e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write payments.json failed: {}", e))?;
    Ok(())
}

fn save_extra_to_json(key: &str, values: &[serde_json::Value]) -> Result<(), String> {
    let settings = get_settings();
    let path = std::path::PathBuf::from(settings.json_data_path).join(key);
    let contents = serde_json::to_string_pretty(values)
        .map_err(|e| format!("Serialize {} failed: {}", key, e))?;
    std::fs::write(&path, contents).map_err(|e| format!("Write {} failed: {}", key, e))?;
    Ok(())
}

fn export_staff_csv(users: &[crate::data::json_store::UserRecord]) -> Result<String, String> {
    let settings = get_settings();
    let export_dir = std::path::PathBuf::from(settings.json_data_path).join("exports");
    std::fs::create_dir_all(&export_dir).map_err(|e| format!("Create export dir failed: {}", e))?;
    let path = export_dir.join("staff_export.csv");
    let mut out = String::from("id,username,email,role,status\n");
    for u in users.iter() {
        out.push_str(&format!("{},{},{},{:?},{}\n", u.id, u.username, u.email, u.role, u.status));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write staff export failed: {}", e))?;
    Ok(path.display().to_string())
}

fn export_customers_csv(customers: &[serde_json::Value]) -> Result<String, String> {
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
        out.push_str(&format!("{},{},{},{},{}\n", id, name, email, segment, status));
    }
    std::fs::write(&path, out).map_err(|e| format!("Write customers export failed: {}", e))?;
    Ok(path.display().to_string())
}

fn pick_first(value: &serde_json::Value, keys: &[&str]) -> String {
    for key in keys {
        if let Some(found) = value_string(value, key) {
            if !found.is_empty() {
                return found;
            }
        }
    }
    "—".to_string()
}

fn value_is(value: &serde_json::Value, key: &str, expected: &str) -> bool {
    value.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

#[component]
fn DashboardTab(user: User) -> Element {
    let store = get_store_fresh();
    let products = &store.products;
    let sales = &store.sales;
    let users = &store.users;
    let sale_items = extra_list("sale_items.json");
    let customers = extra_list("customers.json");
    let payments = extra_list("payments.json");
    let suppliers = extra_list("suppliers.json");
    let shifts = extra_list("shifts.json");
    let roles = extra_list("roles.json");
    let permissions = extra_list("permissions.json");
    let movements = extra_list("inventory_movements.json");

    // compute summary stats from JSON data
    let total_products = products.len();
    let total_stock: i32 = products.iter().map(|p| p.quantity).sum();
    let total_sales_value: f32 = products.iter().map(|p| p.price * p.quantity as f32).sum();
    let active_users = users.iter().filter(|u| u.status == "active").count();
    let inactive_users = users.iter().filter(|u| u.status != "active").count();
    let total_transactions = sales.len();
    let total_revenue: f32 = sales.iter().map(|s| s.total).sum();
    let average_ticket = if total_transactions == 0 {
        0.0
    } else {
        sales.iter().map(|s| s.total).sum::<f32>() / total_transactions as f32
    };

    let total_products_s = format!("{}", total_products);
    let total_stock_s = format!("{}", total_stock);
    let total_sales_s = format_price(total_sales_value);
    let total_revenue_s = format_price(total_revenue);
    let active_users_s = format!("{}", active_users);
    let inactive_users_s = format!("{}", inactive_users);
    let total_transactions_s = format!("{} transactions", total_transactions);
    let average_ticket_s = format!("{} avg ticket", format_price(average_ticket));
    let payments_total: f32 = payments
        .iter()
        .map(|p| value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0))
        .sum();
    let open_shifts = shifts.iter().filter(|s| value_is(s, "status", "open")).count();
    let active_suppliers = suppliers.iter().filter(|s| value_is(s, "status", "active")).count();

    let product_lookup: HashMap<String, &crate::data::models::product::Product> = products
        .iter()
        .map(|p| (p.id.clone(), p))
        .collect();

    let _items_line = format!("Items in products table: {}", total_products_s);
    let _stock_line = format!("Total units in stock: {}", total_stock_s);
    let _sales_line = format!("Estimated total sales value: {}", total_sales_s);

    let mut category_totals: HashMap<String, f32> = HashMap::new();
    for item in sale_items.iter() {
        let product_id = pick_first(item, &["product_id", "productId", "id"]);
        let category = product_lookup
            .get(&product_id)
            .map(|p| p.category.clone())
            .unwrap_or_else(|| "Other".to_string());
        let line_total = value_f32(item, "line_total")
            .or_else(|| value_f32(item, "total"))
            .unwrap_or(0.0);
        *category_totals.entry(category).or_insert(0.0) += line_total;
    }
    let mut category_vec: Vec<(String, f32)> = category_totals.into_iter().collect();
    category_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let max_category = category_vec.first().map(|c| c.1).unwrap_or(1.0);
    let total_category_count: f32 = category_vec.iter().map(|(_, total)| *total).sum::<f32>().max(1.0);
    let top_categories: Vec<(String, i32)> = category_vec
        .iter()
        .take(3)
        .map(|(name, total)| {
            let pct = ((total / total_category_count) * 100.0).round() as i32;
            (name.clone(), pct)
        })
        .collect();
    let categories_s = format!("{}", category_vec.len());
    let p1 = top_categories.get(0).map(|(_, pct)| *pct).unwrap_or(0);
    let p2 = top_categories.get(1).map(|(_, pct)| *pct).unwrap_or(0);
    let p3 = top_categories.get(2).map(|(_, pct)| *pct).unwrap_or(0);
    let _p_rest = (100 - (p1 + p2 + p3)).max(0);
    let donut_style = format!(
        "background: conic-gradient(#6366f1 0 {p1}%, #a78bfa {p1}% {p2_end}%, #22c55e {p2_end}% {p3_end}%, #e2e8f0 {p3_end}% 100%);",
        p1 = p1,
        p2_end = p1 + p2,
        p3_end = p1 + p2 + p3
    );

    let mut spark_values: Vec<f32> = sales.iter().rev().take(8).map(|s| s.total).collect();
    spark_values.reverse();
    let max_spark = spark_values.iter().cloned().fold(0.0, f32::max);
    let spark_heights: Vec<u8> = if spark_values.is_empty() || max_spark <= 0.0 {
        vec![30; 8]
    } else {
        spark_values
            .iter()
            .map(|v| ((v / max_spark) * 70.0 + 15.0).round() as u8)
            .collect()
    };

    // series for chart: aggregate real metrics by day
    let mut sales_by_day: HashMap<String, (f32, i32)> = HashMap::new(); // day -> (revenue, tx count)
    for s in sales.iter() {
        let day = s.created_at.split('T').next().unwrap_or(&s.created_at).to_string();
        let entry = sales_by_day.entry(day).or_insert((0.0, 0));
        entry.0 += s.total;
        entry.1 += 1;
    }
    let mut items_by_day: HashMap<String, i32> = HashMap::new();
    let sales_by_id: HashMap<String, &crate::data::json_store::SaleRecord> = sales
        .iter()
        .map(|s| (s.id.clone(), s))
        .collect();
    for item in sale_items.iter() {
        let sale_id = pick_first(item, &["sale_id", "saleId", "sale"]);
        let day = sales_by_id
            .get(&sale_id)
            .map(|s| s.created_at.split('T').next().unwrap_or(&s.created_at).to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let qty = value_i32(item, "quantity")
            .or_else(|| value_i32(item, "qty"))
            .unwrap_or(0);
        *items_by_day.entry(day).or_insert(0) += qty;
    }
    let mut day_keys: Vec<String> = sales_by_day.keys().cloned().collect();
    day_keys.sort();
    let sales_series: Vec<f32> = day_keys
        .iter()
        .map(|day| sales_by_day.get(day).map(|(v, _)| *v).unwrap_or(0.0))
        .collect();
    let items_series: Vec<f32> = day_keys
        .iter()
        .map(|day| items_by_day.get(day).cloned().unwrap_or(0) as f32)
        .collect();
    let ticket_series: Vec<f32> = day_keys
        .iter()
        .map(|day| {
            sales_by_day
                .get(day)
                .map(|(v, count)| if *count == 0 { 0.0 } else { *v / *count as f32 })
                .unwrap_or(0.0)
        })
        .collect();
    let x_labels: Vec<String> = day_keys.iter().cloned().collect();

    let greeting = format!("Welcome back, {}", user.username);
    let role_label = format!("{}", user.role);
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());
    let profile_initials = {
        let mut chars = user.username.chars().filter(|c| c.is_alphabetic());
        let first = chars.next().unwrap_or('U');
        let second = chars.next().unwrap_or(first);
        format!("{}{}", first, second).to_uppercase()
    };

    let mut product_sales: HashMap<String, (String, i32, f32)> = HashMap::new(); // id -> (name, qty, revenue)
    for item in sale_items.iter() {
        let key = pick_first(item, &["product_id", "productId", "name", "product_name", "id"]);
        let name = pick_first(item, &["name", "product_name", "product_id", "productId", "id"]);
        let qty = value_i32(item, "quantity")
            .or_else(|| value_i32(item, "qty"))
            .unwrap_or(0);
        let revenue = value_f32(item, "line_total")
            .or_else(|| value_f32(item, "total"))
            .unwrap_or(0.0);
        let entry = product_sales.entry(key).or_insert((name, 0, 0.0));
        entry.1 += qty;
        entry.2 += revenue;
    }
    let mut product_sales_vec: Vec<(String, String, i32, f32)> = product_sales
        .into_iter()
        .map(|(id, (name, qty, revenue))| (id, name, qty, revenue))
        .collect();
    product_sales_vec.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
    let _low_stock_count = products.iter().filter(|p| p.quantity < LOW_STOCK_THRESHOLD).count();
    let _out_of_stock = products.iter().filter(|p| p.quantity == 0).count();

    let alerts: Vec<(&str, &str, &str)> = vec![
        ("warning", "Low stock on Croissant", "12 units left"),
        ("info", "Staff shift swap pending", "Review 2 requests"),
        ("success", "Export completed", "Sales_2026-01.csv"),
    ];
    let active_pct = if users.is_empty() { 0 } else { ((active_users as f32 / users.len() as f32) * 100.0).round() as i32 };
    let inactive_pct = 100 - active_pct;
    let initials_for = |name: &str| {
        let mut chars = name.chars().filter(|c| c.is_alphabetic());
        let first = chars.next().unwrap_or('P');
        let second = chars.next().unwrap_or(first);
        format!("{}{}", first, second).to_uppercase()
    };

    let last_day_revenue = day_keys.last().and_then(|d| sales_by_day.get(d)).map(|v| v.0).unwrap_or(0.0);
    let prev_day_revenue = if day_keys.len() > 1 {
        day_keys.get(day_keys.len() - 2).and_then(|d| sales_by_day.get(d)).map(|v| v.0).unwrap_or(0.0)
    } else {
        0.0
    };
    let revenue_delta = if prev_day_revenue <= 0.0 {
        0.0
    } else {
        ((last_day_revenue - prev_day_revenue) / prev_day_revenue) * 100.0
    };
    let revenue_delta_s = format!("{:.1}%", revenue_delta);
    let transactions_today = day_keys.last().and_then(|d| sales_by_day.get(d)).map(|v| v.1).unwrap_or(0);

    let mut top_customers: Vec<(String, i32)> = Vec::new();
    let mut customer_sales: HashMap<String, i32> = HashMap::new();
    for s in sales.iter() {
        if let Some(cid) = s.customer_id.clone() {
            *customer_sales.entry(cid).or_insert(0) += 1;
        }
    }
    for (cid, count) in customer_sales.iter() {
        let name = customers
            .iter()
            .find(|c| pick_first(c, &["id", "customer_id"]) == *cid)
            .map(|c| pick_first(c, &["name", "full_name", "username", "id"]))
            .unwrap_or_else(|| cid.clone());
        top_customers.push((name, *count));
    }
    top_customers.sort_by(|a, b| b.1.cmp(&a.1));

    let mut stats_values: Vec<f32> = sales_series.clone();
    stats_values.reverse();
    stats_values.truncate(12);
    stats_values.reverse();
    let max_stat = stats_values.iter().cloned().fold(0.0, f32::max).max(1.0);
    let stat_heights: Vec<u8> = stats_values
        .iter()
        .map(|v| ((v / max_stat) * 80.0 + 10.0).round() as u8)
        .collect();

    let mut heat_cells: Vec<u8> = Vec::new();
    for i in 0..42 {
        heat_cells.push(((i as i32 * 3 + sales.len() as i32) % 10) as u8);
    }

    rsx! {
        section { class: "dashboard-v2",
            div { class: "dashboard-topbar",
                div { class: "topbar-left",
                    h2 { "Welcome, Admin" }
                    p { "You have {transactions_today} orders today" }
                }
                div { class: "topbar-right",
                    div { class: "topbar-search",
                        span { "🔍" }
                        input { placeholder: "Search", value: "" }
                    }
                    div { class: "topbar-range",
                        span { "📅" }
                        span { "{last_updated}" }
                    }
                }
            }

            div { class: "alert-banner",
                span { "⚠️ Your product Apple iPhone 15 is running low. Add stock." }
                button { class: "link-btn", "Add Stock" }
            }

            div { class: "stat-row",
                div { class: "stat-card-lg accent-orange",
                    span { "Total Sales" }
                    strong { "{total_revenue_s}" }
                    em { "{revenue_delta_s} vs last day" }
                }
                div { class: "stat-card-lg accent-blue",
                    span { "Total Transactions" }
                    strong { "{total_transactions}" }
                    em { "All time" }
                }
                div { class: "stat-card-lg accent-teal",
                    span { "Total Customers" }
                    strong { "{customers.len()}" }
                    em { "{active_users_s} active" }
                }
                div { class: "stat-card-lg accent-indigo",
                    span { "Inventory Value" }
                    strong { "{total_sales_s}" }
                    em { "{total_products_s} products" }
                }
            }

            div { class: "dashboard-grid-main",
                div { class: "content-card chart-card",
                    div { class: "card-header-row",
                        h3 { "Sales & Purchase" }
                        div { class: "pill-group",
                            button { class: "pill-btn", "1D" }
                            button { class: "pill-btn", "1W" }
                            button { class: "pill-btn", "1M" }
                            button { class: "pill-btn active", "1Y" }
                        }
                    }
                    SalesTrend { series: vec![
                        ("Sales".to_string(), sales_series.clone()),
                        ("Items Sold".to_string(), items_series.clone()),
                    ], x_labels: Some(x_labels.clone()) }
                }

                div { class: "content-card overall-card",
                    div { class: "card-header-row",
                        h3 { "Overall Information" }
                        span { class: "card-note", "Key totals" }
                    }
                    div { class: "overall-grid",
                        div { class: "overall-item", strong { "{active_suppliers}" } span { "Suppliers" } }
                        div { class: "overall-item", strong { "{customers.len()}" } span { "Customers" } }
                        div { class: "overall-item", strong { "{total_transactions}" } span { "Orders" } }
                    }
                    div { class: "donut-mini",
                        div { class: "donut-chart", style: "{donut_style}" }
                        div { class: "donut-labels",
                            span { "{categories_s} categories" }
                            span { "{format_price(payments_total)} payments" }
                        }
                    }
                }
            }

            div { class: "dashboard-grid-3",
                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Top Selling Products" }
                        span { class: "card-note", "This month" }
                    }
                    div { class: "data-list",
                        { product_sales_vec.iter().take(5).map(|(_id, name, qty, revenue)| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{name}" }
                                        span { "{qty} sold" }
                                    }
                                    span { class: "data-chip", "{format_price(*revenue)}" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Low Stock Products" }
                        span { class: "card-note", "Need reorder" }
                    }
                    div { class: "data-list",
                        { products.iter().filter(|p| p.quantity < LOW_STOCK_THRESHOLD).take(5).map(|p| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{p.name}" }
                                        span { "{p.category}" }
                                    }
                                    span { class: "data-chip", "{p.quantity} left" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Recent Sales" }
                        span { class: "card-note", "Today" }
                    }
                    div { class: "data-list",
                        { sales.iter().rev().take(5).map(|s| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "Sale {s.id}" }
                                        span { "{format_datetime(&s.created_at)}" }
                                    }
                                    span { class: "data-chip", "{format_price(s.total)}" }
                                }
                            )
                        }) }
                    }
                }
            }

            div { class: "dashboard-grid-2",
                div { class: "content-card stats-card",
                    div { class: "card-header-row",
                        h3 { "Sales Statistics" }
                        span { class: "card-note", "Last 12 periods" }
                    }
                    div { class: "stats-bars",
                        { stat_heights.iter().map(|h| rsx!( span { class: "stats-bar", style: "height: {h}%;" } )) }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Recent Transactions" }
                        span { class: "card-note", "Latest" }
                    }
                    div { class: "data-list",
                        { sales.iter().rev().take(6).map(|s| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{s.cashier_id}" }
                                        span { "Order {s.receipt_no}" }
                                    }
                                    span { class: "data-chip", "{format_price(s.total)}" }
                                }
                            )
                        }) }
                    }
                }
            }

            div { class: "dashboard-grid-3",
                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Top Customers" }
                        span { class: "card-note", "Repeat buyers" }
                    }
                    div { class: "data-list",
                        { top_customers.iter().take(5).map(|(name, count)| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{name}" }
                                        span { "{count} orders" }
                                    }
                                    span { class: "data-chip", "Active" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Top Categories" }
                        span { class: "card-note", "Revenue share" }
                    }
                    div { class: "data-list",
                        { category_vec.iter().take(4).map(|(name, total)| {
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{name}" }
                                        span { "Sales" }
                                    }
                                    span { class: "data-chip", "{format_price(*total)}" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Order Statistics" }
                        span { class: "card-note", "Weekly pattern" }
                    }
                    div { class: "heat-grid",
                        { heat_cells.iter().map(|v| {
                            let cls = format!("heat-cell heat-{}", v);
                            rsx!( div { class: "{cls}" } )
                        }) }
                    }
                }
            }
        }
    }
}

// `SalesTrend` component moved to `src/ui/components/sales_trend.rs`

#[component]
fn StatCard(
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

#[component]
fn PlaceholderTab(title: &'static str) -> Element {
    rsx! {
        div { class: "admin-card",
            div { class: "admin-card-header",
                h3 { "{title}" }
                span { class: "card-subtitle", "This section is ready for data wiring." }
            }
            div { class: "admin-card-body",
                p { "Coming soon." }
            }
        }
    }
}

#[component]
fn SearchInput(placeholder: &'static str) -> Element {
    rsx! {
        div { class: "input-group",
            span { class: "input-icon-left", "🔍" }
            input { r#type: "search", placeholder: "{placeholder}" }
        }
    }
}

fn status_chip_class(status: &str) -> &'static str {
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
fn StatusChip(label: String) -> Element {
    let class_name = format!("{}", status_chip_class(&label));
    rsx! {
        span { class: "{class_name}", "{label}" }
    }
}

#[component]
fn RowActions() -> Element {
    rsx! {
        div { class: "table-actions",
            button { class: "btn-secondary", "View" }
            button { class: "btn-secondary", "Edit" }
            button { class: "btn-danger", "Delete" }
        }
    }
}

#[component]
fn SalesDashboardTab() -> Element {
    let store = get_store_fresh();
    let sales = &store.sales;
    let sale_items = extra_list("sale_items.json");
    let customers = extra_list("customers.json");
    let payments = extra_list("payments.json");
    let products = &store.products;

    let total_transactions = sales.len();
    let total_revenue: f32 = sales.iter().map(|s| s.total).sum();
    let average_ticket = if total_transactions == 0 {
        0.0
    } else {
        total_revenue / total_transactions as f32
    };
    let voided_sales = sales.iter().filter(|s| s.status.eq_ignore_ascii_case("voided")).count();
    let paid_sales = sales.iter().filter(|s| s.status.eq_ignore_ascii_case("paid") || s.status.eq_ignore_ascii_case("completed")).count();
    let pending_sales = total_transactions.saturating_sub(paid_sales + voided_sales);
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());

    let mut sales_by_day: HashMap<String, (f32, i32)> = HashMap::new();
    for s in sales.iter() {
        let day = s.created_at.split('T').next().unwrap_or(&s.created_at).to_string();
        let entry = sales_by_day.entry(day).or_insert((0.0, 0));
        entry.0 += s.total;
        entry.1 += 1;
    }
    let mut items_by_day: HashMap<String, i32> = HashMap::new();
    let sales_by_id: HashMap<String, &crate::data::json_store::SaleRecord> = sales
        .iter()
        .map(|s| (s.id.clone(), s))
        .collect();
    for item in sale_items.iter() {
        let sale_id = pick_first(item, &["sale_id", "saleId", "sale"]);
        let day = sales_by_id
            .get(&sale_id)
            .map(|s| s.created_at.split('T').next().unwrap_or(&s.created_at).to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let qty = value_i32(item, "quantity")
            .or_else(|| value_i32(item, "qty"))
            .unwrap_or(0);
        *items_by_day.entry(day).or_insert(0) += qty;
    }
    let mut day_keys: Vec<String> = sales_by_day.keys().cloned().collect();
    day_keys.sort();
    let sales_series: Vec<f32> = day_keys
        .iter()
        .map(|day| sales_by_day.get(day).map(|(v, _)| *v).unwrap_or(0.0))
        .collect();
    let items_series: Vec<f32> = day_keys
        .iter()
        .map(|day| items_by_day.get(day).cloned().unwrap_or(0) as f32)
        .collect();
    let x_labels: Vec<String> = day_keys.iter().cloned().collect();

    let mut customer_sales: HashMap<String, i32> = HashMap::new();
    for s in sales.iter() {
        if let Some(cid) = s.customer_id.clone() {
            *customer_sales.entry(cid).or_insert(0) += 1;
        }
    }
    let mut top_customers: Vec<(String, i32)> = Vec::new();
    for (cid, count) in customer_sales.iter() {
        let name = customers
            .iter()
            .find(|c| pick_first(c, &["id", "customer_id"]) == *cid)
            .map(|c| pick_first(c, &["name", "full_name", "username", "id"]))
            .unwrap_or_else(|| cid.clone());
        top_customers.push((name, *count));
    }
    top_customers.sort_by(|a, b| b.1.cmp(&a.1));

    let mut product_sales: HashMap<String, (String, i32, f32)> = HashMap::new();
    let product_lookup: HashMap<String, &Product> = products.iter().map(|p| (p.id.clone(), p)).collect();
    for item in sale_items.iter() {
        let product_id = pick_first(item, &["product_id", "productId", "id"]);
        let name = product_lookup
            .get(&product_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| pick_first(item, &["name", "product_name", "product_id", "productId", "id"]));
        let qty = value_i32(item, "quantity")
            .or_else(|| value_i32(item, "qty"))
            .unwrap_or(0);
        let revenue = value_f32(item, "line_total")
            .or_else(|| value_f32(item, "total"))
            .unwrap_or(0.0);
        let entry = product_sales.entry(product_id).or_insert((name, 0, 0.0));
        entry.1 += qty;
        entry.2 += revenue;
    }
    let mut product_sales_vec: Vec<(String, String, i32, f32)> = product_sales
        .into_iter()
        .map(|(id, (name, qty, revenue))| (id, name, qty, revenue))
        .collect();
    product_sales_vec.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    let mut payment_rows: Vec<(String, String, String)> = Vec::new();
    for p in payments.iter().rev().take(5) {
        let method = pick_first(p, &["method", "type", "channel"]);
        let amount = value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0);
        let date = pick_first(p, &["paid_at", "created_at", "date"]);
        payment_rows.push((method, format_price(amount), date));
    }

    rsx! {
        section { class: "dashboard-v2",
            div { class: "dashboard-topbar",
                div { class: "topbar-left",
                    h2 { "Sales Dashboard" }
                    p { "Track revenue, orders, and top performers." }
                }
                div { class: "topbar-right",
                    div { class: "topbar-range",
                        span { "📅" }
                        span { "{last_updated}" }
                    }
                }
            }

            div { class: "stat-row",
                div { class: "stat-card-lg accent-orange",
                    span { "Total Revenue" }
                    strong { "{format_price(total_revenue)}" }
                    em { "{total_transactions} transactions" }
                }
                div { class: "stat-card-lg accent-blue",
                    span { "Average Ticket" }
                    strong { "{format_price(average_ticket)}" }
                    em { "{paid_sales} paid" }
                }
                div { class: "stat-card-lg accent-teal",
                    span { "Pending Sales" }
                    strong { "{pending_sales}" }
                    em { "Awaiting settlement" }
                }
                div { class: "stat-card-lg accent-indigo",
                    span { "Voided Sales" }
                    strong { "{voided_sales}" }
                    em { "Requires review" }
                }
            }

            div { class: "dashboard-grid-main",
                div { class: "content-card chart-card",
                    div { class: "card-header-row",
                        h3 { "Sales Trend" }
                        span { class: "card-note", "Revenue + items" }
                    }
                    SalesTrend { series: vec![
                        ("Revenue".to_string(), sales_series.clone()),
                        ("Items Sold".to_string(), items_series.clone()),
                    ], x_labels: Some(x_labels.clone()) }
                }

                div { class: "content-card overall-card",
                    div { class: "card-header-row",
                        h3 { "Sales Status" }
                        span { class: "card-note", "Breakdown" }
                    }
                    div { class: "overall-grid",
                        div { class: "overall-item", strong { "{paid_sales}" } span { "Paid" } }
                        div { class: "overall-item", strong { "{pending_sales}" } span { "Pending" } }
                        div { class: "overall-item", strong { "{voided_sales}" } span { "Voided" } }
                    }
                }
            }

            div { class: "dashboard-grid-3",
                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Top Customers" }
                        span { class: "card-note", "Orders" }
                    }
                    div { class: "data-list",
                        if top_customers.is_empty() {
                            div { class: "empty-state", "No customer sales yet." }
                        } else {
                            { top_customers.iter().take(6).map(|(name, count)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{count} orders" }
                                        }
                                        span { class: "data-chip", "{count}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Top Products" }
                        span { class: "card-note", "Revenue" }
                    }
                    div { class: "data-list",
                        if product_sales_vec.is_empty() {
                            div { class: "empty-state", "No product sales yet." }
                        } else {
                            { product_sales_vec.iter().take(6).map(|(_id, name, qty, revenue)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{qty} sold" }
                                        }
                                        span { class: "data-chip", "{format_price(*revenue)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "content-card list-card",
                    div { class: "card-header-row",
                        h3 { "Recent Payments" }
                        span { class: "card-note", "Last 5" }
                    }
                    div { class: "data-list",
                        if payment_rows.is_empty() {
                            div { class: "empty-state", "No payments yet." }
                        } else {
                            { payment_rows.iter().map(|(method, amount, date)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{method}" }
                                            span { "{date}" }
                                        }
                                        span { class: "data-chip", "{amount}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductsTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_product_id = use_signal(|| None::<String>);
    let mut product_query = use_signal(|| String::new());
    let mut product_sort = use_signal(|| "name".to_string());
    let mut product_order = use_signal(|| "asc".to_string());
    let mut product_page = use_signal(|| 1usize);
    let mut product_name = use_signal(|| String::new());
    let mut product_barcode = use_signal(|| String::new());
    let mut product_category = use_signal(|| String::new());
    let mut product_price = use_signal(|| String::new());
    let mut product_stock = use_signal(|| String::new());
    let mut product_image_preview = use_signal(|| None::<String>);
    let mut product_image_payload = use_signal(|| None::<String>);
    let mut product_image_type = use_signal(|| None::<String>);
    let mut edit_product_name = use_signal(|| String::new());
    let mut edit_product_barcode = use_signal(|| String::new());
    let mut edit_product_category = use_signal(|| String::new());
    let mut edit_product_price = use_signal(|| String::new());
    let mut edit_product_stock = use_signal(|| String::new());
    let mut edit_product_image_preview = use_signal(|| None::<String>);
    let mut edit_product_image_payload = use_signal(|| None::<String>);
    let mut edit_product_image_name = use_signal(|| None::<String>);
    let mut edit_product_image_type = use_signal(|| None::<String>);
    let store = get_store_fresh();
    let mut products_state = use_signal(|| store.products.clone());
    let mut edit_save_msg = use_signal(|| false);
    let mut edit_error = use_signal(|| None::<String>);
    let mut add_save_msg = use_signal(|| false);
    let mut add_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);
    let mut product_export_msg = use_signal(|| None::<String>);
    let mut product_export_error = use_signal(|| None::<String>);
    let products = products_state.read().clone();
    let users = &store.users;
    let sales = &store.sales;

    let total_products = products.len();
    let total_stock: i32 = products.iter().map(|p| p.quantity).sum();
    let active_users = users.iter().filter(|u| u.status == "active").count();
    let inactive_users = users.iter().filter(|u| u.status != "active").count();
    let total_transactions = sales.len();
    let total_revenue: f32 = sales.iter().map(|s| s.total).sum();
    let average_ticket = if total_transactions == 0 {
        0.0
    } else {
        total_revenue / total_transactions as f32
    };

    let total_products_s = format!("{}", total_products);
    let total_stock_s = format!("{}", total_stock);
    let active_users_s = format!("{}", active_users);
    let inactive_users_s = format!("{}", inactive_users);
    let total_transactions_s = format!("{} transactions", total_transactions);
    let total_revenue_s = format_price(total_revenue);
    let average_ticket_s = format!("{} avg ticket", format_price(average_ticket));
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());
    
    let handle_add_product = {
        let mut products_state = products_state.clone();
        let mut add_error = add_error.clone();
        let mut add_save_msg = add_save_msg.clone();
        let mut show_add_modal = show_add_modal.clone();
        let mut product_name = product_name.clone();
        let mut product_barcode = product_barcode.clone();
        let mut product_category = product_category.clone();
        let mut product_price = product_price.clone();
        let mut product_stock = product_stock.clone();
        let mut product_image_preview = product_image_preview.clone();
        let mut product_image_payload = product_image_payload.clone();
        let mut product_image_type = product_image_type.clone();
        move |_| {
            if product_name.read().is_empty() || product_barcode.read().is_empty() {
                add_error.set(Some("Product name and barcode are required.".to_string()));
                return;
            }
            let price = match product_price.read().parse::<f32>() {
                Ok(v) if v >= 0.01 => v,
                _ => {
                    add_error.set(Some("Price must be at least 0.01.".to_string()));
                    return;
                }
            };
            let stock = match product_stock.read().parse::<i32>() {
                Ok(v) if v >= 0 => v,
                _ => {
                    add_error.set(Some("Stock must be a non-negative integer.".to_string()));
                    return;
                }
            };
            let mut updated = products_state.read().clone();
            if updated.iter().any(|p| p.barcode == *product_barcode.read()) {
                add_error.set(Some("A product with this barcode already exists.".to_string()));
                return;
            }
            let mut image_filename: Option<String> = None;
            let mut image_type: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (product_image_payload.read().clone(), product_image_type.read().clone()) {
                match ImageService::save_product_image(&payload, &img_type) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type = Some(img_type);
                    }
                    Err(err) => {
                        add_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            let product = Product {
                id: new_id("prod"),
                barcode: product_barcode.read().clone(),
                name: product_name.read().clone(),
                description: None,
                price,
                quantity: stock,
                category: if product_category.read().is_empty() { "Uncategorized".to_string() } else { product_category.read().clone() },
                product_image: image_filename,
                product_image_type: image_type,
                created_at: now.clone(),
                updated_at: now,
            };
            updated.push(product);
            match save_products_to_json(&updated) {
                Ok(_) => {
                    products_state.set(updated);
                    add_error.set(None);
                    add_save_msg.set(true);
                    product_name.set(String::new());
                    product_barcode.set(String::new());
                    product_category.set(String::new());
                    product_price.set(String::new());
                    product_stock.set(String::new());
                    product_image_preview.set(None);
                    product_image_payload.set(None);
                    product_image_type.set(None);
                    show_add_modal.set(false);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_save_msg.set(false);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };
    
    // Render rows from JSON products inline (rsx expects an iterator)
    let mut products_sorted: Vec<_> = products.iter().cloned().collect();
    products_sorted.sort_by(|a, b| {
        let a_val = a.price * a.quantity as f32;
        let b_val = b.price * b.quantity as f32;
        b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
    });
    let low_stock_items: Vec<_> = products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .cloned()
        .collect();
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock_count = low_stock_items.len();

    let handle_edit_product = move |_| {
        let id = match edit_product_id.read().clone() {
            Some(id) => id,
            None => return,
        };
        let price = edit_product_price.read().parse::<f32>();
        let stock = edit_product_stock.read().parse::<i32>();
        if price.is_err() || stock.is_err() {
            edit_error.set(Some("Invalid price or stock value.".to_string()));
            return;
        }
        let price = price.unwrap_or(0.0);
        let stock = stock.unwrap_or(0);
        if price < 0.01 {
            edit_error.set(Some("Price must be at least 0.01.".to_string()));
            return;
        }
        if stock < 0 {
            edit_error.set(Some("Stock must be a non-negative integer.".to_string()));
            return;
        }

        let mut updated = products_state.read().clone();
        if updated.iter().any(|p| p.barcode == *edit_product_barcode.read() && p.id != id) {
            edit_error.set(Some("Another product already uses this barcode.".to_string()));
            return;
        }
        let mut image_filename = edit_product_image_name.read().clone();
        let mut image_type = edit_product_image_type.read().clone();
        if let (Some(payload), Some(img_type)) = (edit_product_image_payload.read().clone(), edit_product_image_type.read().clone()) {
            match ImageService::save_product_image(&payload, &img_type) {
                Ok(filename) => {
                    image_filename = Some(filename);
                    image_type = Some(img_type);
                }
                Err(err) => {
                    edit_error.set(Some(format!("Image upload failed: {}", err)));
                    return;
                }
            }
        }
        if let Some(p) = updated.iter_mut().find(|p| p.id == id) {
            p.name = edit_product_name.read().clone();
            p.barcode = edit_product_barcode.read().clone();
            p.category = edit_product_category.read().clone();
            p.price = price;
            p.quantity = stock;
            p.product_image = image_filename;
            p.product_image_type = image_type;
        } else {
            edit_error.set(Some("Product not found for update.".to_string()));
            return;
        }

        match save_products_to_json(&updated) {
            Ok(_) => {
                products_state.set(updated);
                edit_error.set(None);
                edit_save_msg.set(true);
                show_edit_modal.set(false);
                edit_product_id.set(None);
                spawn(async move {
                    sleep_ms(2_000).await;
                    edit_save_msg.set(false);
                });
            }
            Err(err) => {
                edit_error.set(Some(err));
            }
        }
    };

    let query = product_query.read().to_lowercase();
    let mut filtered_products: Vec<Product> = products
        .clone()
        .into_iter()
        .filter(|p| {
            if query.is_empty() {
                true
            } else {
                let name = p.name.to_lowercase();
                let barcode = p.barcode.to_lowercase();
                let category = p.category.to_lowercase();
                name.contains(&query) || barcode.contains(&query) || category.contains(&query)
            }
        })
        .collect();
    let sort_key = product_sort.read().clone();
    filtered_products.sort_by(|a, b| {
        match sort_key.as_str() {
            "stock" => a.quantity.cmp(&b.quantity),
            "price" => a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal),
            "value" => {
                let av = a.price * a.quantity as f32;
                let bv = b.price * b.quantity as f32;
                av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
            }
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    if product_order.read().as_str() == "desc" {
        filtered_products.reverse();
    }
    let page_size = 10usize;
    let total_pages = std::cmp::max(1, (filtered_products.len() + page_size - 1) / page_size);
    let current_page = (*product_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_products.len());
    let page_items: Vec<Product> = if filtered_products.is_empty() {
        Vec::new()
    } else {
        filtered_products[start..end].to_vec()
    };

    let mut product_rows: Vec<Element> = Vec::new();
    for p in page_items.into_iter() {
        let status = if p.quantity == 0 { "Out" } else if p.quantity < LOW_STOCK_THRESHOLD { "Low" } else { "OK" };
        let status_class = if p.quantity == 0 { "status-chip danger" } else if p.quantity < LOW_STOCK_THRESHOLD { "status-chip warning" } else { "status-chip ok" };
        let pid = p.id.clone();
        let pname = p.name.clone();
        let pbarcode = p.barcode.clone();
        let pcategory = p.category.clone();
        let pprice = p.price;
        let pqty = p.quantity;
        let pimage_name = p.product_image.clone();
        let pimage_type = p.product_image_type.clone();
        let mut edit_product_id = edit_product_id.clone();
        let mut edit_product_name = edit_product_name.clone();
        let mut edit_product_barcode = edit_product_barcode.clone();
        let mut edit_product_category = edit_product_category.clone();
        let mut edit_product_price = edit_product_price.clone();
        let mut edit_product_stock = edit_product_stock.clone();
        let mut edit_product_image_preview = edit_product_image_preview.clone();
        let mut edit_product_image_payload = edit_product_image_payload.clone();
        let mut edit_product_image_name = edit_product_image_name.clone();
        let mut edit_product_image_type = edit_product_image_type.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        let mut products_state = products_state.clone();
        let mut delete_msg = delete_msg.clone();
        let mut delete_error = delete_error.clone();
        let pid_for_delete = pid.clone();
        product_rows.push(rsx! {
            tr {
                td { "{pbarcode}" }
                td { "{pname}" }
                td { "{pcategory}" }
                td { "{format_price(pprice)}" }
                td { "{pqty}" }
                td { span { class: "{status_class}", "{status}" } }
                td { div { class: "table-actions",
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            edit_product_id.set(Some(pid.clone()));
                            edit_product_name.set(pname.clone());
                            edit_product_barcode.set(pbarcode.clone());
                            edit_product_category.set(pcategory.clone());
                            edit_product_price.set(format!("{:.2}", pprice));
                            edit_product_stock.set(format!("{}", pqty));
                            edit_product_image_name.set(pimage_name.clone());
                            edit_product_image_type.set(pimage_type.clone());
                            edit_product_image_payload.set(None);
                            let preview = pimage_name
                                .as_deref()
                                .and_then(|name| image_preview_from_filename(name, &pimage_type, PRODUCTS_IMAGES_DIR));
                            edit_product_image_preview.set(preview);
                            show_edit_modal.set(true);
                        },
                        "✏️ Edit"
                    }
                    button {
                        class: "btn-small btn-danger",
                        onclick: move |_| {
                            let mut updated = products_state.read().clone();
                            updated.retain(|prod| prod.id != pid_for_delete);
                            match save_products_to_json(&updated) {
                                Ok(_) => {
                                    products_state.set(updated);
                                    delete_error.set(None);
                                    delete_msg.set(Some("✅ Product deleted.".to_string()));
                                    spawn(async move {
                                        sleep_ms(2_000).await;
                                        delete_msg.set(None);
                                    });
                                }
                                Err(err) => delete_error.set(Some(err)),
                            }
                        },
                        "🗑️ Delete"
                    }
                } }
            }
        });
    }

    rsx! {
        div { class: "content-card",
            div { class: "products-header",
                h2 { "📦 Manage Products" }
                div { class: "products-header-actions",
                    span { class: "ops-pill warning", "Low stock < {LOW_STOCK_THRESHOLD}" }
                    div { class: "filter-group filter-inline",
                        label { "Search" }
                        input {
                            placeholder: "Name, barcode, category",
                            value: "{product_query}",
                            oninput: move |e| {
                                product_query.set(e.value());
                                product_page.set(1);
                            },
                        }
                    }
                    div { class: "filter-group filter-inline",
                        label { "Sort" }
                        select {
                            value: "{product_sort}",
                            onchange: move |e| product_sort.set(e.value()),
                            option { value: "name", "Name" }
                            option { value: "stock", "Stock" }
                            option { value: "price", "Price" }
                            option { value: "value", "Value" }
                        }
                    }
                    div { class: "filter-group filter-inline",
                        label { "Order" }
                        select {
                            value: "{product_order}",
                            onchange: move |e| product_order.set(e.value()),
                            option { value: "asc", "Asc" }
                            option { value: "desc", "Desc" }
                        }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_add_modal.set(true),
                        "+ Add Product"
                    }
                }
            }
            
            if edit_save_msg.read().clone() {
                div { class: "message",
                    "✅ Product updated successfully!"
                }
            }
            if let Some(err) = edit_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if add_save_msg.read().clone() {
                div { class: "message",
                    "✅ Product created successfully!"
                }
            }
            if let Some(err) = add_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = delete_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = delete_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = product_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = product_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Barcode" }
                            th { "Name" }
                            th { "Category" }
                            th { "Price" }
                            th { "Stock" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if product_rows.is_empty() {
                            tr { td { colspan: "7",
                                div { class: "empty-state", "No products match your filters." }
                            } }
                        } else {
                            { product_rows.into_iter() }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_products_csv(&products) {
                            Ok(path) => {
                                product_export_error.set(None);
                                product_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => product_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| product_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| product_page.set(current_page + 1),
                    "Next"
                }
            }
            
            if show_add_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "➕ Add New Product" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_add_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Product Name *" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    placeholder: "e.g., Apple",
                                    value: "{product_name}",
                                    oninput: move |e| product_name.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Barcode *" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    placeholder: "e.g., 123456789012",
                                    value: "{product_barcode}",
                                    oninput: move |e| product_barcode.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    placeholder: "e.g., Fruits",
                                    value: "{product_category}",
                                    oninput: move |e| product_category.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Price ($)" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "💲" }
                                input {
                                    placeholder: "e.g., 1.50",
                                    value: "{product_price}",
                                    oninput: move |e| product_price.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input {
                                    placeholder: "e.g., 100",
                                    value: "{product_stock}",
                                    oninput: move |e| product_stock.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Product Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut product_image_payload = product_image_payload.clone();
                                  let mut product_image_type = product_image_type.clone();
                                  let mut product_image_preview = product_image_preview.clone();
                                  let mut add_error = add_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    product_image_payload.set(Some(payload));
                                                    product_image_type.set(Some(img_type));
                                                    product_image_preview.set(Some(preview));
                                                }
                                                Err(err) => {
                                                    add_error.set(Some(format!("Image load failed: {}", err)));
                                                }
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = product_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_add_product,
                                "✅ Save Product"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_add_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Product" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Product Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    value: "{edit_product_name}",
                                    oninput: move |e| edit_product_name.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Barcode" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    value: "{edit_product_barcode}",
                                    oninput: move |e| edit_product_barcode.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    value: "{edit_product_category}",
                                    oninput: move |e| edit_product_category.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Price ($)" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "💲" }
                                input {
                                    value: "{edit_product_price}",
                                    oninput: move |e| edit_product_price.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input {
                                    value: "{edit_product_stock}",
                                    oninput: move |e| edit_product_stock.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Product Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut edit_product_image_payload = edit_product_image_payload.clone();
                                  let mut edit_product_image_type = edit_product_image_type.clone();
                                  let mut edit_product_image_preview = edit_product_image_preview.clone();
                                  let mut edit_error = edit_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    edit_product_image_payload.set(Some(payload));
                                                    edit_product_image_type.set(Some(img_type));
                                                    edit_product_image_preview.set(Some(preview));
                                                }
                                                Err(err) => {
                                                    edit_error.set(Some(format!("Image load failed: {}", err)));
                                                }
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = edit_product_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_product,
                                "✅ Save Changes"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }

            div { class: "data-ops-grid",
                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Users Snapshot" }
                        span { class: "card-note", "{active_users_s} active · {inactive_users_s} inactive" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total users" }
                            strong { "{users.len()}" }
                        }
                        div { class: "data-stat",
                            span { "Active" }
                            strong { "{active_users_s}" }
                        }
                        div { class: "data-stat",
                            span { "Inactive" }
                            strong { "{inactive_users_s}" }
                        }
                    }
                    div { class: "data-list",
                        { users.iter().take(4).map(|u| {
                            let role_label = match u.role {
                                crate::data::models::user::UserRole::Admin => "Admin",
                                crate::data::models::user::UserRole::Staff => "Staff",
                            };
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{u.username}" }
                                        span { "{u.email}" }
                                    }
                                    span { class: "data-chip", "{role_label}" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Products Snapshot" }
                        span { class: "card-note", "{total_products_s} items" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total stock" }
                            strong { "{total_stock_s}" }
                        }
                        div { class: "data-stat",
                            span { "Low stock" }
                            strong { "{low_stock_count}" }
                        }
                        div { class: "data-stat",
                            span { "Out of stock" }
                            strong { "{out_of_stock}" }
                        }
                    }
                    div { class: "data-list",
                        { products_sorted.iter().take(4).map(|p| {
                            let value = format_price(p.price);
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{p.name}" }
                                        span { "{p.category} · {value}" }
                                    }
                                    span { class: "data-chip", "{p.quantity} in stock" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Sales Snapshot" }
                        span { class: "card-note", "{total_transactions_s}" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total revenue" }
                            strong { "{total_revenue_s}" }
                        }
                        div { class: "data-stat",
                            span { "Avg ticket" }
                            strong { "{average_ticket_s}" }
                        }
                        div { class: "data-stat",
                            span { "Last update" }
                            strong { "{last_updated}" }
                        }
                    }
                    div { class: "data-list",
                        { sales.iter().rev().take(4).map(|s| {
                            let amount = format_price(s.total);
                            let when = format_datetime(&s.created_at);
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "Sale {s.id}" }
                                        span { "{when}" }
                                    }
                                    span { class: "data-chip", "{amount}" }
                                }
                            )
                        }) }
                    }
                }
            }
        }
    }
}

#[component]
fn SuperAdminTab() -> Element {
    let store = get_store_fresh();
    let mut stores_state = use_signal(|| extra_list("stores.json").iter().map(|v| (*v).clone()).collect::<Vec<_>>());
    let mut plans_state = use_signal(|| extra_list("plans.json").iter().map(|v| (*v).clone()).collect::<Vec<_>>());
    let mut audits_state = use_signal(|| extra_list("system_audits.json").iter().map(|v| (*v).clone()).collect::<Vec<_>>());
    let mut notifications_state = use_signal(|| extra_list("global_notifications.json").iter().map(|v| (*v).clone()).collect::<Vec<_>>());

    let mut store_name = use_signal(|| String::new());
    let mut store_owner = use_signal(|| String::new());
    let mut store_email = use_signal(|| String::new());
    let mut store_phone = use_signal(|| String::new());
    let mut store_status = use_signal(|| "Active".to_string());

    let mut plan_name = use_signal(|| String::new());
    let mut plan_price = use_signal(|| String::new());
    let mut plan_cycle = use_signal(|| String::new());

    let mut notice_title = use_signal(|| String::new());
    let mut notice_body = use_signal(|| String::new());

    let mut action_msg = use_signal(|| None::<String>);
    let mut action_error = use_signal(|| None::<String>);

    let handle_create_store = {
        let mut stores_state = stores_state.clone();
        let mut store_name = store_name.clone();
        let mut store_owner = store_owner.clone();
        let mut store_email = store_email.clone();
        let mut store_phone = store_phone.clone();
        let mut store_status = store_status.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if store_name.read().is_empty() {
                action_error.set(Some("Store name is required.".to_string()));
                return;
            }
            let mut updated = stores_state.read().clone();
            let now = now_iso();
            updated.push(json!({
                "id": new_id("st"),
                "store": store_name.read().clone(),
                "user": store_owner.read().clone(),
                "email": store_email.read().clone(),
                "phone": store_phone.read().clone(),
                "status": store_status.read().clone(),
                "created_at": now,
            }));
            match save_extra_to_json("stores.json", &updated) {
                Ok(_) => {
                    stores_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Store created.".to_string()));
                    store_name.set(String::new());
                    store_owner.set(String::new());
                    store_email.set(String::new());
                    store_phone.set(String::new());
                    store_status.set("Active".to_string());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_add_plan = {
        let mut plans_state = plans_state.clone();
        let mut plan_name = plan_name.clone();
        let mut plan_price = plan_price.clone();
        let mut plan_cycle = plan_cycle.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if plan_name.read().is_empty() {
                action_error.set(Some("Plan name is required.".to_string()));
                return;
            }
            let mut updated = plans_state.read().clone();
            updated.push(json!({
                "id": new_id("plan"),
                "name": plan_name.read().clone(),
                "price": plan_price.read().clone(),
                "cycle": plan_cycle.read().clone(),
                "created_at": now_iso(),
            }));
            match save_extra_to_json("plans.json", &updated) {
                Ok(_) => {
                    plans_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Plan added.".to_string()));
                    plan_name.set(String::new());
                    plan_price.set(String::new());
                    plan_cycle.set(String::new());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_run_audit = {
        let mut audits_state = audits_state.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            let mut updated = audits_state.read().clone();
            updated.push(json!({
                "id": new_id("audit"),
                "title": "System audit",
                "status": "completed",
                "created_at": now_iso(),
            }));
            match save_extra_to_json("system_audits.json", &updated) {
                Ok(_) => {
                    audits_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ System audit logged.".to_string()));
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_add_notification = {
        let mut notifications_state = notifications_state.clone();
        let mut notice_title = notice_title.clone();
        let mut notice_body = notice_body.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if notice_title.read().is_empty() || notice_body.read().is_empty() {
                action_error.set(Some("Notification title and message are required.".to_string()));
                return;
            }
            let mut updated = notifications_state.read().clone();
            updated.push(json!({
                "id": new_id("note"),
                "title": notice_title.read().clone(),
                "message": notice_body.read().clone(),
                "created_at": now_iso(),
            }));
            match save_extra_to_json("global_notifications.json", &updated) {
                Ok(_) => {
                    notifications_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Notification queued.".to_string()));
                    notice_title.set(String::new());
                    notice_body.set(String::new());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "admin-form-page",
            if let Some(msg) = action_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = action_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Create Store" }
                    span { class: "card-subtitle", "Provision a new store and owner details." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Store Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏪" }
                            input {
                                placeholder: "Downtown Outlet",
                                value: "{store_name}",
                                oninput: move |e| store_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Owner / Manager" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "👤" }
                            input {
                                placeholder: "manager01",
                                value: "{store_owner}",
                                oninput: move |e| store_owner.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "store@example.com",
                                value: "{store_email}",
                                oninput: move |e| store_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0100",
                                value: "{store_phone}",
                                oninput: move |e| store_phone.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Status" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{store_status}",
                                onchange: move |e| store_status.set(e.value()),
                                option { value: "Active", "Active" }
                                option { value: "Inactive", "Inactive" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_create_store,
                        "Create Store"
                    }
                }
                div { class: "data-list",
                    { stores_state.read().iter().take(6).map(|s| {
                        let name = pick_first(s, &["store", "name"]);
                        let owner = pick_first(s, &["user", "owner"]);
                        let status = pick_first(s, &["status"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{name}" }
                                    span { "{owner}" }
                                }
                                span { class: "data-chip", "{status}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Manage Plans" }
                    span { class: "card-subtitle", "Add or adjust subscription plans." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Plan Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏷️" }
                            input {
                                placeholder: "Growth",
                                value: "{plan_name}",
                                oninput: move |e| plan_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Price" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "💲" }
                            input {
                                placeholder: "$49",
                                value: "{plan_price}",
                                oninput: move |e| plan_price.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Billing Cycle" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🗓️" }
                            input {
                                placeholder: "Monthly",
                                value: "{plan_cycle}",
                                oninput: move |e| plan_cycle.set(e.value()),
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-secondary",
                        onclick: handle_add_plan,
                        "Add Plan"
                    }
                }
                div { class: "data-list",
                    { plans_state.read().iter().take(6).map(|p| {
                        let name = pick_first(p, &["name", "title"]);
                        let price = pick_first(p, &["price", "amount"]);
                        let cycle = pick_first(p, &["cycle", "billing"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{name}" }
                                    span { "{cycle}" }
                                }
                                span { class: "data-chip", "{price}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "System Audit" }
                    span { class: "card-subtitle", "Log a system audit run." }
                }
                div { class: "admin-card-body grid-actions",
                    button { class: "btn-secondary", onclick: handle_run_audit, "Run System Audit" }
                }
                div { class: "data-list",
                    { audits_state.read().iter().rev().take(5).map(|a| {
                        let title = pick_first(a, &["title", "name"]);
                        let status = pick_first(a, &["status"]);
                        let date = pick_first(a, &["created_at", "created"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{title}" }
                                    span { "{date}" }
                                }
                                span { class: "data-chip", "{status}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Global Notifications" }
                    span { class: "card-subtitle", "Push announcements to all stores." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Title" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📣" }
                            input {
                                placeholder: "Holiday Hours Update",
                                value: "{notice_title}",
                                oninput: move |e| notice_title.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Message" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            textarea {
                                rows: "3",
                                placeholder: "All stores close at 6 PM this Friday.",
                                value: "{notice_body}",
                                oninput: move |e| notice_body.set(e.value()),
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-secondary",
                        onclick: handle_add_notification,
                        "Send Notification"
                    }
                }
                div { class: "data-list",
                    { notifications_state.read().iter().rev().take(5).map(|n| {
                        let title = pick_first(n, &["title", "name"]);
                        let message = pick_first(n, &["message", "body"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{title}" }
                                    span { "{message}" }
                                }
                            }
                        )
                    }) }
                }
            }
        }
    }
}

#[component]
fn CreateProductTab() -> Element {
    let store = get_store_fresh();
    let categories = extra_list("categories.json");
    let sub_categories = extra_list("sub_categories.json");
    let brands = extra_list("brands.json");
    let units = extra_list("units.json");
    let warranties = extra_list("warranties.json");
    let warehouses = extra_list("warehouses.json");
    let stores = extra_list("stores.json");
    let mut products_state = use_signal(|| store.products.clone());
    let mut save_msg = use_signal(|| false);
    let mut save_error = use_signal(|| None::<String>);
    let mut store_name = use_signal(|| String::new());
    let mut warehouse_name = use_signal(|| String::new());
    let mut product_name = use_signal(|| String::new());
    let mut sku = use_signal(|| String::new());
    let mut category = use_signal(|| String::new());
    let mut sub_category = use_signal(|| String::new());
    let mut brand = use_signal(|| String::new());
    let mut unit = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut quantity = use_signal(|| String::new());
    let mut price = use_signal(|| String::new());
    let mut tax_type = use_signal(|| String::new());
    let mut discount = use_signal(|| String::new());
    let mut alert_qty = use_signal(|| String::new());
    let mut warranty = use_signal(|| String::new());
    let mut manufacturer = use_signal(|| String::new());
    let mut manufactured_date = use_signal(|| String::new());
    let mut expiry_date = use_signal(|| String::new());
    let mut image_payload = use_signal(|| None::<String>);
    let mut image_type = use_signal(|| None::<String>);
    let mut image_preview = use_signal(|| None::<String>);

    let handle_save = {
        let mut products_state = products_state.clone();
        let mut save_msg = save_msg.clone();
        let mut save_error = save_error.clone();
        let mut product_name = product_name.clone();
        let mut sku = sku.clone();
        let mut category = category.clone();
        let mut description = description.clone();
        let mut quantity = quantity.clone();
        let mut price = price.clone();
        let mut store_name = store_name.clone();
        let mut warehouse_name = warehouse_name.clone();
        let mut sub_category = sub_category.clone();
        let mut brand = brand.clone();
        let mut unit = unit.clone();
        let mut tax_type = tax_type.clone();
        let mut discount = discount.clone();
        let mut alert_qty = alert_qty.clone();
        let mut warranty = warranty.clone();
        let mut manufacturer = manufacturer.clone();
        let mut manufactured_date = manufactured_date.clone();
        let mut expiry_date = expiry_date.clone();
        let mut image_payload = image_payload.clone();
        let mut image_type = image_type.clone();
        let mut image_preview = image_preview.clone();
        move |_| {
            if product_name.read().is_empty() || sku.read().is_empty() {
                save_error.set(Some("Product name and SKU are required.".to_string()));
                return;
            }
            let price_val = match price.read().parse::<f32>() {
                Ok(v) if v >= 0.01 => v,
                Err(_) => {
                    save_error.set(Some("Price must be a valid number.".to_string()));
                    return;
                }
                _ => {
                    save_error.set(Some("Price must be at least 0.01.".to_string()));
                    return;
                }
            };
            let qty_val = match quantity.read().parse::<i32>() {
                Ok(v) if v >= 0 => v,
                Err(_) => {
                    save_error.set(Some("Quantity must be a valid number.".to_string()));
                    return;
                }
                _ => {
                    save_error.set(Some("Quantity must be a non-negative number.".to_string()));
                    return;
                }
            };
            let mut updated = products_state.read().clone();
            if updated.iter().any(|p| p.barcode == *sku.read()) {
                save_error.set(Some("A product with this SKU/Barcode already exists.".to_string()));
                return;
            }
            let mut image_filename: Option<String> = None;
            let mut image_type_saved: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (image_payload.read().clone(), image_type.read().clone()) {
                match ImageService::save_product_image(&payload, &img_type) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        save_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            let product = Product {
                id: new_id("prod"),
                barcode: sku.read().clone(),
                name: product_name.read().clone(),
                description: if description.read().is_empty() { None } else { Some(description.read().clone()) },
                price: price_val,
                quantity: qty_val,
                category: if category.read().is_empty() { "Uncategorized".to_string() } else { category.read().clone() },
                product_image: image_filename,
                product_image_type: image_type_saved,
                created_at: now.clone(),
                updated_at: now,
            };
            updated.push(product);
            match save_products_to_json(&updated) {
                Ok(_) => {
                    products_state.set(updated);
                    save_error.set(None);
                    save_msg.set(true);
                    product_name.set(String::new());
                    sku.set(String::new());
                    category.set(String::new());
                    sub_category.set(String::new());
                    brand.set(String::new());
                    unit.set(String::new());
                    description.set(String::new());
                    quantity.set(String::new());
                    price.set(String::new());
                    tax_type.set(String::new());
                    discount.set(String::new());
                    alert_qty.set(String::new());
                    warranty.set(String::new());
                    manufacturer.set(String::new());
                    manufactured_date.set(String::new());
                    expiry_date.set(String::new());
                    store_name.set(String::new());
                    warehouse_name.set(String::new());
                    image_payload.set(None);
                    image_type.set(None);
                    image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        save_msg.set(false);
                    });
                }
                Err(err) => save_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "admin-form-page",
            if save_msg.read().clone() {
                div { class: "message",
                    "✅ Product created successfully!"
                }
            }
            if let Some(err) = save_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Product Information" }
                    span { class: "card-subtitle", "Fill in the core details for a new product." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Store" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{store_name}",
                                onchange: move |e| store_name.set(e.value()),
                                option { value: "", "Select" }
                                { stores.iter().map(|s| {
                                    let name = pick_first(s, &["store", "name", "title"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Warehouse" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{warehouse_name}",
                                onchange: move |e| warehouse_name.set(e.value()),
                                option { value: "", "Select" }
                                { warehouses.iter().map(|w| {
                                    let name = pick_first(w, &["warehouse", "name", "title"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Product Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            input {
                                r#type: "text",
                                placeholder: "Apple iPhone 15",
                                value: "{product_name}",
                                oninput: move |e| product_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "SKU" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏷️" }
                            input {
                                r#type: "text",
                                placeholder: "PT001",
                                value: "{sku}",
                                oninput: move |e| sku.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Category" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{category}",
                                onchange: move |e| category.set(e.value()),
                                option { value: "", "Select" }
                                { categories.iter().map(|c| {
                                    let name = pick_first(c, &["name", "category"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Sub Category" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{sub_category}",
                                onchange: move |e| sub_category.set(e.value()),
                                option { value: "", "Select" }
                                { sub_categories.iter().map(|c| {
                                    let name = pick_first(c, &["sub_category", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Brand" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{brand}",
                                onchange: move |e| brand.set(e.value()),
                                option { value: "", "Select" }
                                { brands.iter().map(|b| {
                                    let name = pick_first(b, &["brand", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Unit" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{unit}",
                                onchange: move |e| unit.set(e.value()),
                                option { value: "", "Select" }
                                { units.iter().map(|u| {
                                    let name = pick_first(u, &["unit", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Description" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            textarea {
                                rows: "4",
                                placeholder: "Short product description...",
                                value: "{description}",
                                oninput: move |e| description.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Pricing & Stocks" }
                    span { class: "card-subtitle", "Set pricing, taxes, and inventory alerts." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Quantity" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🔢" }
                            input {
                                r#type: "number",
                                placeholder: "100",
                                value: "{quantity}",
                                oninput: move |e| quantity.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Price" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "💲" }
                            input {
                                r#type: "number",
                                placeholder: "$120.00",
                                value: "{price}",
                                oninput: move |e| price.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Tax Type" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{tax_type}",
                                onchange: move |e| tax_type.set(e.value()),
                                option { value: "", "Select" }
                                option { value: "inclusive", "Inclusive" }
                                option { value: "exclusive", "Exclusive" }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Discount" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "％" }
                            input {
                                r#type: "text",
                                placeholder: "5%",
                                value: "{discount}",
                                oninput: move |e| discount.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Alert Quantity" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "⚠️" }
                            input {
                                r#type: "number",
                                placeholder: "10",
                                value: "{alert_qty}",
                                oninput: move |e| alert_qty.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Images" }
                    span { class: "card-subtitle", "Upload product imagery and gallery assets." }
                }
                div { class: "admin-card-body",
                    div { class: "upload-drop",
                        span { "Drag & drop or click to upload images" }
                        { let mut image_payload = image_payload.clone();
                          let mut image_type = image_type.clone();
                          let mut image_preview = image_preview.clone();
                          let mut save_error = save_error.clone();
                          rsx!(
                            input {
                                r#type: "file",
                                accept: "image/*",
                                onchange: move |e| {
                                    let path = e.value();
                                    if path.is_empty() {
                                        return;
                                    }
                                    match load_image_from_path(&path) {
                                        Ok((payload, img_type, preview)) => {
                                            image_payload.set(Some(payload));
                                            image_type.set(Some(img_type));
                                            image_preview.set(Some(preview));
                                        }
                                        Err(err) => {
                                            save_error.set(Some(format!("Image load failed: {}", err)));
                                        }
                                    }
                                },
                            }
                          )
                        }
                    }
                    if let Some(img) = image_preview.read().clone() {
                        div { class: "image-preview",
                            img { src: "{img}" }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Custom Fields" }
                    span { class: "card-subtitle", "Warranty, manufacturer, and expiry data." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Warranty" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{warranty}",
                                onchange: move |e| warranty.set(e.value()),
                                option { value: "", "Select" }
                                { warranties.iter().map(|w| {
                                    let name = pick_first(w, &["warranty", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Manufacturer" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏭" }
                            input {
                                r#type: "text",
                                placeholder: "Acme Corp",
                                value: "{manufacturer}",
                                oninput: move |e| manufacturer.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Manufactured Date" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "📅" }
                            input {
                                r#type: "date",
                                value: "{manufactured_date}",
                                oninput: move |e| manufactured_date.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Expiry Date" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "📅" }
                            input {
                                r#type: "date",
                                value: "{expiry_date}",
                                oninput: move |e| expiry_date.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "form-actions",
                button { class: "btn-secondary", "Cancel" }
                button {
                    class: "btn-primary",
                    onclick: handle_save,
                    "Add Product"
                }
            }
        }
    }
}

#[component]
fn ExpiredProductsTab() -> Element {
    let store = get_store_fresh();
    let expired = extra_list("expired_products.json");
    let rows: Vec<(String, String, String, String, String)> = expired
        .iter()
        .map(|item| {
            let sku = pick_first(item, &["sku", "code"]);
            let product = pick_first(item, &["product", "name"]);
            let manufactured = pick_first(item, &["manufactured", "manufactured_at"]);
            let expired_at = pick_first(item, &["expired", "expired_at"]);
            let status = "Expired".to_string();
            (sku, product, manufactured, expired_at, status)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search expired products..." }
                div { class: "toolbar-actions",
                    button { class: "btn-secondary", "Export" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Manufactured" }
                            th { "Expired" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, manufactured, expired_at, status) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{manufactured}" }
                                td { "{expired_at}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LowStocksTab() -> Element {
    let store = get_store_fresh();
    let low_products: Vec<(String, String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .map(|p| {
            let status = if p.quantity == 0 { "Out" } else { "Low" };
            (p.barcode.clone(), p.name.clone(), p.category.clone(), p.quantity.to_string(), status.to_string())
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search low stock..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Send Email" }
                    button { class: "btn-secondary", "Notify" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Category" }
                            th { "Qty" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, name, category, qty, status) in low_products {
                            tr {
                                td { "{sku}" }
                                td { "{name}" }
                                td { "{category}" }
                                td { "{qty}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CategoriesTab() -> Element {
    let store = get_store_fresh();
    let mut categories_state = use_signal(|| {
        extra_list("categories.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut slug_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());

    let rows = categories_state.read().clone();
    let open_new = move |_| {
        edit_id.set(None);
        name_state.set(String::new());
        slug_state.set(String::new());
        status_state.set("Active".to_string());
        show_modal.set(true);
    };

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search categories..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: open_new, "Add Category" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Category" }
                            th { "Category Slug" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "slug"]);
                            let name = pick_first(item, &["name", "category"]);
                            let slug = pick_first(item, &["slug", "id"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let status = if status == "—" { "Active".to_string() } else { status };
                            let edit_id_clone = id.clone();
                            let item_clone = item.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["name", "category"]));
                                slug_state.set(pick_first(&item_clone, &["slug", "id"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = categories_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "slug"]) != id);
                                if save_extra_to_json("categories.json", &next).is_ok() {
                                    categories_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{slug}" }
                                    td { "{created}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Category\" } else { \"Add Category\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Category Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    r#type: "text",
                                    value: "{name_state.read()}",
                                    oninput: move |e| name_state.set(e.value.clone()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Slug" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    r#type: "text",
                                    value: "{slug_state.read()}",
                                    oninput: move |e| slug_state.set(e.value.clone()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select {
                                    value: "{status_state.read()}",
                                    onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button {
                                class: "btn-primary",
                                onclick: move |_| {
                                    let mut next = categories_state.read().clone();
                                    let id = edit_id.read().clone().unwrap_or_else(|| {
                                        format!("cat{}", next.len() + 1)
                                    });
                                    let value = json!({
                                        "id": id,
                                        "name": name_state.read().clone(),
                                        "slug": slug_state.read().clone(),
                                        "created_at": "2026-01-01",
                                        "status": status_state.read().clone(),
                                    });
                                    if let Some(edit) = edit_id.read().clone() {
                                        if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "slug"]) == edit) {
                                            *entry = value;
                                        }
                                    } else {
                                        next.push(value);
                                    }
                                    if save_extra_to_json("categories.json", &next).is_ok() {
                                        categories_state.set(next);
                                        show_modal.set(false);
                                    }
                                },
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SubCategoriesTab() -> Element {
    let store = get_store_fresh();
    let mut subcats_state = use_signal(|| {
        extra_list("sub_categories.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut category_state = use_signal(|| String::new());
    let mut code_state = use_signal(|| String::new());
    let mut description_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());

    let rows = subcats_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search sub categories..." }
                div { class: "toolbar-actions",
                    button {
                        class: "btn-primary",
                        onclick: move |_| {
                            edit_id.set(None);
                            name_state.set(String::new());
                            category_state.set(String::new());
                            code_state.set(String::new());
                            description_state.set(String::new());
                            status_state.set("Active".to_string());
                            show_modal.set(true);
                        },
                        "Add Sub Category"
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Sub Category" }
                            th { "Category" }
                            th { "Category Code" }
                            th { "Description" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "code"]);
                            let name = pick_first(item, &["sub_category", "name"]);
                            let category = pick_first(item, &["category"]);
                            let code = pick_first(item, &["code", "id"]);
                            let description = pick_first(item, &["description"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["sub_category", "name"]));
                                category_state.set(pick_first(&item_clone, &["category"]));
                                code_state.set(pick_first(&item_clone, &["code", "id"]));
                                description_state.set(pick_first(&item_clone, &["description"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = subcats_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "code"]) != id);
                                if save_extra_to_json("sub_categories.json", &next).is_ok() {
                                    subcats_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{category}" }
                                    td { "{code}" }
                                    td { "{description}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Sub Category\" } else { \"Add Sub Category\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Sub Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input { r#type: "text", value: "{category_state.read()}", oninput: move |e| category_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Code" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "text", value: "{code_state.read()}", oninput: move |e| code_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Description" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input { r#type: "text", value: "{description_state.read()}", oninput: move |e| description_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = subcats_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("SC{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "sub_category": name_state.read().clone(),
                                    "category": category_state.read().clone(),
                                    "code": code_state.read().clone(),
                                    "description": description_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "code"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("sub_categories.json", &next).is_ok() {
                                    subcats_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BrandsTab() -> Element {
    let store = get_store_fresh();
    let mut brands_state = use_signal(|| {
        extra_list("brands.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = brands_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search brands..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Brand" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Brand" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "brand"]);
                            let name = pick_first(item, &["brand", "name"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["brand", "name"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = brands_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "brand"]) != id);
                                if save_extra_to_json("brands.json", &next).is_ok() {
                                    brands_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{created}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Brand\" } else { \"Add Brand\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Brand Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = brands_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("BR{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "brand": name_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "brand"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("brands.json", &next).is_ok() {
                                    brands_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UnitsTab() -> Element {
    let store = get_store_fresh();
    let mut units_state = use_signal(|| {
        extra_list("units.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut short_state = use_signal(|| String::new());
    let mut products_state = use_signal(|| String::from("0"));
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = units_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search units..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        short_state.set(String::new());
                        products_state.set("0".to_string());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Unit" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Unit" }
                            th { "Short Name" }
                            th { "No of Products" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "unit"]);
                            let name = pick_first(item, &["unit", "name"]);
                            let short = pick_first(item, &["short", "abbr"]);
                            let products = pick_first(item, &["products", "count"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["unit", "name"]));
                                short_state.set(pick_first(&item_clone, &["short", "abbr"]));
                                products_state.set(pick_first(&item_clone, &["products", "count"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = units_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "unit"]) != id);
                                if save_extra_to_json("units.json", &next).is_ok() {
                                    units_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{short}" }
                                    td { "{products}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Unit\" } else { \"Add Unit\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Unit Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Short Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔤" }
                                input { r#type: "text", value: "{short_state.read()}", oninput: move |e| short_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "No of Products" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "number", value: "{products_state.read()}", oninput: move |e| products_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = units_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("U{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "unit": name_state.read().clone(),
                                    "short": short_state.read().clone(),
                                    "products": products_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "unit"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("units.json", &next).is_ok() {
                                    units_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VariantAttributesTab() -> Element {
    let store = get_store_fresh();
    let mut variants_state = use_signal(|| {
        extra_list("variant_attributes.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut values_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = variants_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search variants..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        values_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Variant" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Variant" }
                            th { "Values" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "variant"]);
                            let name = pick_first(item, &["variant", "name"]);
                            let values = pick_first(item, &["values", "options"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["variant", "name"]));
                                values_state.set(pick_first(&item_clone, &["values", "options"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = variants_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "variant"]) != id);
                                if save_extra_to_json("variant_attributes.json", &next).is_ok() {
                                    variants_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{values}" }
                                    td { "{created}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Variant\" } else { \"Add Variant\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Variant" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🧩" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Values" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "text", value: "{values_state.read()}", oninput: move |e| values_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = variants_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("VA{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "variant": name_state.read().clone(),
                                    "values": values_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "variant"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("variant_attributes.json", &next).is_ok() {
                                    variants_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn WarrantiesTab() -> Element {
    let store = get_store_fresh();
    let mut warranties_state = use_signal(|| {
        extra_list("warranties.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut description_state = use_signal(|| String::new());
    let mut duration_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = warranties_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search warranties..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        description_state.set(String::new());
                        duration_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Warranty" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Warranty" }
                            th { "Description" }
                            th { "Duration" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "warranty"]);
                            let name = pick_first(item, &["warranty", "name"]);
                            let description = pick_first(item, &["description"]);
                            let duration = pick_first(item, &["duration"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["warranty", "name"]));
                                description_state.set(pick_first(&item_clone, &["description"]));
                                duration_state.set(pick_first(&item_clone, &["duration"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = warranties_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "warranty"]) != id);
                                if save_extra_to_json("warranties.json", &next).is_ok() {
                                    warranties_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{description}" }
                                    td { "{duration}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Warranty\" } else { \"Add Warranty\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Warranty" }
                            input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                        }
                        div { class: "form-group",
                            label { "Description" }
                            input { r#type: "text", value: "{description_state.read()}", oninput: move |e| description_state.set(e.value.clone()) }
                        }
                        div { class: "form-group",
                            label { "Duration" }
                            input { r#type: "text", value: "{duration_state.read()}", oninput: move |e| duration_state.set(e.value.clone()) }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                option { value: "Active", "Active" }
                                option { value: "Inactive", "Inactive" }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = warranties_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("W{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "warranty": name_state.read().clone(),
                                    "description": description_state.read().clone(),
                                    "duration": duration_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "warranty"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("warranties.json", &next).is_ok() {
                                    warranties_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PrintBarcodeTab() -> Element {
    rsx! {
        div { class: "admin-form-page",
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Print Barcode" }
                    span { class: "card-subtitle", "Generate barcode labels for selected products." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Warehouse" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field",
                        label { "Store" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field span-2",
                        label { "Product" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🔍" }
                            input { r#type: "search", placeholder: "Search product by code" }
                        }
                    }
                }
                div { class: "table-container table-compact",
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Product" }
                                th { "SKU" }
                                th { "Code" }
                                th { "Qty" }
                            }
                        }
                        tbody {
                            tr {
                                td { colspan: "4",
                                    div { class: "empty-state", "No Data Available" }
                                }
                            }
                        }
                    }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Paper Size" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Store Name" }
                        input { r#type: "checkbox", checked: true }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Product Name" }
                        input { r#type: "checkbox", checked: true }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Price" }
                        input { r#type: "checkbox", checked: true }
                    }
                }
                div { class: "form-actions",
                    button { class: "btn-secondary", "Reset Barcode" }
                    button { class: "btn-primary", "Print Barcode" }
                }
            }
        }
    }
}

#[component]
fn PrintQrCodeTab() -> Element {
    rsx! {
        div { class: "admin-form-page",
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Print QR Code" }
                    span { class: "card-subtitle", "Create QR codes for inventory labels." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Warehouse" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field",
                        label { "Store" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field span-2",
                        label { "Product" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🔍" }
                            input { r#type: "search", placeholder: "Search product by code" }
                        }
                    }
                }
                div { class: "table-container table-compact",
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Product" }
                                th { "SKU" }
                                th { "Code" }
                                th { "Reference Number" }
                                th { "Qty" }
                            }
                        }
                        tbody {
                            tr {
                                td { colspan: "5",
                                    div { class: "empty-state", "No Data Available" }
                                }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button { class: "btn-secondary", "Reset Barcode" }
                    button { class: "btn-primary", "Print Barcode" }
                }
            }
        }
    }
}

#[component]
fn WarehousesTab() -> Element {
    let store = get_store_fresh();
    let mut warehouses_state = use_signal(|| {
        extra_list("warehouses.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut contact_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut total_products_state = use_signal(|| String::from("0"));
    let mut stock_state = use_signal(|| String::from("0"));
    let mut qty_state = use_signal(|| String::from("0"));
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = warehouses_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search warehouse..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        contact_state.set(String::new());
                        phone_state.set(String::new());
                        total_products_state.set("0".to_string());
                        stock_state.set("0".to_string());
                        qty_state.set("0".to_string());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Warehouse" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Warehouse" }
                            th { "Contact Person" }
                            th { "Phone" }
                            th { "Total Products" }
                            th { "Stock" }
                            th { "Qty" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "warehouse"]);
                            let warehouse = pick_first(item, &["warehouse", "name"]);
                            let contact = pick_first(item, &["contact", "contact_person"]);
                            let phone = pick_first(item, &["phone"]);
                            let total_products = pick_first(item, &["total_products"]);
                            let stock = pick_first(item, &["stock"]);
                            let qty = pick_first(item, &["qty", "quantity"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["warehouse", "name"]));
                                contact_state.set(pick_first(&item_clone, &["contact", "contact_person"]));
                                phone_state.set(pick_first(&item_clone, &["phone"]));
                                total_products_state.set(pick_first(&item_clone, &["total_products"]));
                                stock_state.set(pick_first(&item_clone, &["stock"]));
                                qty_state.set(pick_first(&item_clone, &["qty", "quantity"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = warehouses_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "warehouse"]) != id);
                                if save_extra_to_json("warehouses.json", &next).is_ok() {
                                    warehouses_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{warehouse}" }
                                    td { "{contact}" }
                                    td { "{phone}" }
                                    td { "{total_products}" }
                                    td { "{stock}" }
                                    td { "{qty}" }
                                    td { "{created}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Warehouse\" } else { \"Add Warehouse\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Warehouse" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏬" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Contact Person" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{contact_state.read()}", oninput: move |e| contact_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input { r#type: "text", value: "{phone_state.read()}", oninput: move |e| phone_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Total Products" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "number", value: "{total_products_state.read()}", oninput: move |e| total_products_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "number", value: "{stock_state.read()}", oninput: move |e| stock_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Qty" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "number", value: "{qty_state.read()}", oninput: move |e| qty_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = warehouses_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("wh{}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "warehouse": name_state.read().clone(),
                                    "contact": contact_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "total_products": total_products_state.read().clone(),
                                    "stock": stock_state.read().clone(),
                                    "qty": qty_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "warehouse"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("warehouses.json", &next).is_ok() {
                                    warehouses_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StoresTab() -> Element {
    let store = get_store_fresh();
    let mut stores_state = use_signal(|| {
        extra_list("stores.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut user_state = use_signal(|| String::new());
    let mut email_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = stores_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search stores..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        user_state.set(String::new());
                        email_state.set(String::new());
                        phone_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Store" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Store" }
                            th { "User Name" }
                            th { "Email" }
                            th { "Phone" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "store"]);
                            let store_name = pick_first(item, &["store", "name"]);
                            let user = pick_first(item, &["user", "username"]);
                            let email = pick_first(item, &["email"]);
                            let phone = pick_first(item, &["phone"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["store", "name"]));
                                user_state.set(pick_first(&item_clone, &["user", "username"]));
                                email_state.set(pick_first(&item_clone, &["email"]));
                                phone_state.set(pick_first(&item_clone, &["phone"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = stores_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "store"]) != id);
                                if save_extra_to_json("stores.json", &next).is_ok() {
                                    stores_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{store_name}" }
                                    td { "{user}" }
                                    td { "{email}" }
                                    td { "{phone}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Store\" } else { \"Add Store\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Store Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏬" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "User Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{user_state.read()}", oninput: move |e| user_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input { r#type: "email", value: "{email_state.read()}", oninput: move |e| email_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input { r#type: "text", value: "{phone_state.read()}", oninput: move |e| phone_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = stores_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("st{}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "store": name_state.read().clone(),
                                    "user": user_state.read().clone(),
                                    "email": email_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "store"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("stores.json", &next).is_ok() {
                                    stores_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BillersTab() -> Element {
    let store = get_store_fresh();
    let mut billers_state = use_signal(|| {
        extra_list("billers.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut biller_state = use_signal(|| String::new());
    let mut company_state = use_signal(|| String::new());
    let mut email_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut country_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = billers_state.read().clone();
    let row_nodes: Vec<Element> = rows
        .iter()
        .map(|item| {
            let code = pick_first(item, &["id", "code"]);
            let biller = pick_first(item, &["biller", "name"]);
            let company = pick_first(item, &["company", "company_name"]);
            let email = pick_first(item, &["email"]);
            let phone = pick_first(item, &["phone"]);
            let country = pick_first(item, &["country"]);
            let status = pick_first(item, &["status"]);
            let item_clone = item.clone();
            let edit_id_clone = code.clone();
            let on_edit = move |_| {
                edit_id.set(Some(edit_id_clone.clone()));
                biller_state.set(pick_first(&item_clone, &["biller", "name"]));
                company_state.set(pick_first(&item_clone, &["company", "company_name"]));
                email_state.set(pick_first(&item_clone, &["email"]));
                phone_state.set(pick_first(&item_clone, &["phone"]));
                country_state.set(pick_first(&item_clone, &["country"]));
                status_state.set(pick_first(&item_clone, &["status"]));
                show_modal.set(true);
            };
            let on_delete = move |_| {
                let mut next = billers_state.read().clone();
                next.retain(|v| pick_first(v, &["id", "code"]) != code);
                if save_extra_to_json("billers.json", &next).is_ok() {
                    billers_state.set(next);
                }
            };
            rsx!(
                tr {
                    td { "{code}" }
                    td { "{biller}" }
                    td { "{company}" }
                    td { "{email}" }
                    td { "{phone}" }
                    td { "{country}" }
                    td { StatusChip { label: status } }
                    td {
                        div { class: "table-actions",
                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                        }
                    }
                }
            )
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search billers..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        biller_state.set(String::new());
                        company_state.set(String::new());
                        email_state.set(String::new());
                        phone_state.set(String::new());
                        country_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Biller" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Code" }
                            th { "Biller" }
                            th { "Company Name" }
                            th { "Email" }
                            th { "Phone" }
                            th { "Country" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody { { row_nodes } }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Biller\" } else { \"Add Biller\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Biller Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{biller_state.read()}", oninput: move |e| biller_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Company" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏢" }
                                input { r#type: "text", value: "{company_state.read()}", oninput: move |e| company_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input { r#type: "email", value: "{email_state.read()}", oninput: move |e| email_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input { r#type: "text", value: "{phone_state.read()}", oninput: move |e| phone_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Country" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🌍" }
                                input { r#type: "text", value: "{country_state.read()}", oninput: move |e| country_state.set(e.value.clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value.clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = billers_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("BL{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "biller": biller_state.read().clone(),
                                    "company": company_state.read().clone(),
                                    "email": email_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "country": country_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "code"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("billers.json", &next).is_ok() {
                                    billers_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ManageStockTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let user_lookup: HashMap<String, String> = store
        .users
        .iter()
        .map(|u| (u.id.clone(), u.username.clone()))
        .collect();

    let rows: Vec<(String, String, String, String, String)> = movements
        .iter()
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let staff_id = pick_first(item, &["staff_id"]);
            let product = product_lookup.get(&product_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let staff = user_lookup.get(&staff_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let date = pick_first(item, &["created_at"]);
            let movement = pick_first(item, &["movement_type"]);
            (product, staff, movement, qty, date)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search stock movements..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Add Brand" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Person" }
                            th { "Movement" }
                            th { "Qty" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, staff, movement, qty, date) in rows {
                            tr {
                                td { "{product}" }
                                td { "{staff}" }
                                td { "{movement}" }
                                td { "{qty}" }
                                td { "{date}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StockAdjustmentTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let rows: Vec<(String, String, String, String)> = movements
        .iter()
        .filter(|item| value_is(item, "movement_type", "adjust"))
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let product = product_lookup.get(&product_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let reason = pick_first(item, &["reason"]);
            let date = pick_first(item, &["created_at"]);
            (product, qty, reason, date)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search adjustments..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Add Adjustment" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Qty" }
                            th { "Reason" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, qty, reason, date) in rows {
                            tr {
                                td { "{product}" }
                                td { "{qty}" }
                                td { "{reason}" }
                                td { "{date}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StockTransferTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let rows: Vec<(String, String, String, String)> = movements
        .iter()
        .filter(|item| value_is(item, "movement_type", "out"))
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let product = product_lookup.get(&product_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let reason = pick_first(item, &["reason"]);
            let date = pick_first(item, &["created_at"]);
            (product, qty, reason, date)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search transfers..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Add Transfer" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Qty" }
                            th { "Reason" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, qty, reason, date) in rows {
                            tr {
                                td { "{product}" }
                                td { "{qty}" }
                                td { "{reason}" }
                                td { "{date}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InvoiceReportTab() -> Element {
    let store = get_store_fresh();
    let rows: Vec<(String, String, String, String, String)> = store
        .sales
        .iter()
        .map(|s| {
            let customer = s.customer_id.clone().unwrap_or_else(|| "Walk-in".to_string());
            let amount = format_price(s.total);
            let paid = if s.status.to_lowercase() == "paid" { amount.clone() } else { "$0.00".to_string() };
            let due = if s.status.to_lowercase() == "paid" { "$0.00".to_string() } else { amount.clone() };
            (s.receipt_no.clone(), customer, amount, paid, s.status.clone())
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "kpi-row",
                div { class: "kpi-card success",
                    span { "Total Amount" }
                    strong { "{format_price(store.sales.iter().map(|s| s.total).sum())}" }
                }
                div { class: "kpi-card info",
                    span { "Total Paid" }
                    strong { "{format_price(store.sales.iter().filter(|s| s.status == \"paid\").map(|s| s.total).sum())}" }
                }
                div { class: "kpi-card warning",
                    span { "Total Unpaid" }
                    strong { "{format_price(store.sales.iter().filter(|s| s.status != \"paid\").map(|s| s.total).sum())}" }
                }
                div { class: "kpi-card danger",
                    span { "Overdue" }
                    strong { "{format_price(store.sales.iter().filter(|s| s.status != \"paid\").map(|s| s.total).sum())}" }
                }
            }
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search invoices..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Invoice" }
                            th { "Customer" }
                            th { "Amount" }
                            th { "Paid" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (invoice, customer, amount, paid, status) in rows {
                            tr {
                                td { "{invoice}" }
                                td { "{customer}" }
                                td { "{amount}" }
                                td { "{paid}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SupplierReportTab() -> Element {
    let store = get_store_fresh();
    let suppliers = extra_list("suppliers.json");
    let rows: Vec<(String, String, String, String, String)> = suppliers
        .iter()
        .map(|item| {
            let reference = pick_first(item, &["id", "reference"]);
            let supplier = pick_first(item, &["name", "supplier"]);
            let items = pick_first(item, &["items", "total_items"]);
            let amount = pick_first(item, &["amount", "total", "balance"]);
            let status = pick_first(item, &["status"]);
            (reference, supplier, items, amount, status)
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search suppliers..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Reference" }
                            th { "Supplier" }
                            th { "Total Items" }
                            th { "Amount" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (reference, supplier, items, amount, status) in rows {
                            tr {
                                td { "{reference}" }
                                td { "{supplier}" }
                                td { "{items}" }
                                td { "{amount}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CustomerReportTab() -> Element {
    let store = get_store_fresh();
    let customers = extra_list("customers.json");
    let mut orders_by_customer: HashMap<String, i32> = HashMap::new();
    for sale in store.sales.iter() {
        if let Some(customer_id) = &sale.customer_id {
            *orders_by_customer.entry(customer_id.clone()).or_insert(0) += 1;
        }
    }
    let rows: Vec<(String, String, String, String, String)> = customers
        .iter()
        .map(|item| {
            let id = pick_first(item, &["id", "customer_id"]);
            let customer = pick_first(item, &["name", "full_name", "username"]);
            let orders = orders_by_customer.get(&id).cloned().unwrap_or(0).to_string();
            let amount = pick_first(item, &["amount", "total"]);
            let status = pick_first(item, &["status", "payment_status"]);
            (id, customer, orders, amount, status)
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search customers..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Reference" }
                            th { "Customer" }
                            th { "Total Orders" }
                            th { "Amount" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (reference, customer, orders, amount, status) in rows {
                            tr {
                                td { "{reference}" }
                                td { "{customer}" }
                                td { "{orders}" }
                                td { "{amount}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ProductReportTab() -> Element {
    let store = get_store_fresh();
    let mut category_filter = use_signal(|| "All".to_string());
    let mut brand_filter = use_signal(|| "All".to_string());
    let sale_items = extra_list("sale_items.json");
    let mut totals: HashMap<String, (i32, f32)> = HashMap::new();
    for item in sale_items.iter() {
        let product_id = pick_first(item, &["product_id", "productId", "id"]);
        let qty = value_i32(item, "quantity").unwrap_or(0);
        let total = value_f32(item, "line_total").or_else(|| value_f32(item, "total")).unwrap_or(0.0);
        let entry = totals.entry(product_id).or_insert((0, 0.0));
        entry.0 += qty;
        entry.1 += total;
    }
    let mut categories: Vec<String> = store.products.iter().map(|p| p.category.clone()).collect();
    categories.sort();
    categories.dedup();
    let rows: Vec<(String, String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| {
            let cat_ok = category_filter.read().as_str() == "All" || p.category == *category_filter.read();
            let brand_ok = brand_filter.read().as_str() == "All"
                || p.name.to_lowercase().contains(&brand_filter.read().to_lowercase());
            cat_ok && brand_ok
        })
        .map(|p| {
            let (qty, total) = totals.get(&p.id).cloned().unwrap_or((0, 0.0));
            (p.barcode.clone(), p.name.clone(), p.category.clone(), qty.to_string(), format_price(total))
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search products..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "report-filters",
                div { class: "form-field",
                    label { "Category" }
                    select { value: "{category_filter.read()}", onchange: move |e| category_filter.set(e.value.clone()),
                        option { value: "All", "All" }
                        for cat in categories.iter() {
                            option { value: "{cat}", "{cat}" }
                        }
                    }
                }
                div { class: "form-field",
                    label { "Brand" }
                    select { value: "{brand_filter.read()}", onchange: move |e| brand_filter.set(e.value.clone()),
                        option { value: "All", "All" }
                        option { value: "Lenovo", "Lenovo" }
                        option { value: "Apple", "Apple" }
                        option { value: "Nike", "Nike" }
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Category" }
                            th { "Qty" }
                            th { "Revenue" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, category, qty, revenue) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{category}" }
                                td { "{qty}" }
                                td { "{revenue}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryReportTab() -> Element {
    let store = get_store_fresh();
    let mut category_filter = use_signal(|| "All".to_string());
    let mut categories: Vec<String> = store.products.iter().map(|p| p.category.clone()).collect();
    categories.sort();
    categories.dedup();
    let rows: Vec<(String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| category_filter.read().as_str() == "All" || p.category == *category_filter.read())
        .map(|p| (p.barcode.clone(), p.name.clone(), p.category.clone(), p.quantity.to_string()))
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search inventory..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "report-filters",
                div { class: "form-field",
                    label { "Category" }
                    select { value: "{category_filter.read()}", onchange: move |e| category_filter.set(e.value.clone()),
                        option { value: "All", "All" }
                        for cat in categories.iter() {
                            option { value: "{cat}", "{cat}" }
                        }
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Category" }
                            th { "In Stock" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, category, qty) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{category}" }
                                td { "{qty}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PurchaseReportTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let rows: Vec<(String, String, String, String)> = movements
        .iter()
        .filter(|item| value_is(item, "movement_type", "in"))
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let product = product_lookup.get(&product_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let date = pick_first(item, &["created_at"]);
            let amount = pick_first(item, &["amount", "total"]);
            (product, qty, date, amount)
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search purchases..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Purchase Qty" }
                            th { "Purchase Date" }
                            th { "Amount" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, qty, date, amount) in rows {
                            tr {
                                td { "{product}" }
                                td { "{qty}" }
                                td { "{date}" }
                                td { "{amount}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SalesReportTab() -> Element {
    let store = get_store_fresh();
    let mut start_date = use_signal(|| String::new());
    let mut end_date = use_signal(|| String::new());
    let mut status_filter = use_signal(|| "All".to_string());

    let rows: Vec<(String, String, String, String)> = store
        .sales
        .iter()
        .filter(|s| {
            let day = s.created_at.split('T').next().unwrap_or(&s.created_at);
            let status_ok = status_filter.read().as_str() == "All"
                || s.status.eq_ignore_ascii_case(status_filter.read().as_str());
            let start_ok = start_date.read().is_empty() || day >= start_date.read().as_str();
            let end_ok = end_date.read().is_empty() || day <= end_date.read().as_str();
            status_ok && start_ok && end_ok
        })
        .map(|s| {
            let customer = s.customer_id.clone().unwrap_or_else(|| "Walk-in".to_string());
            let amount = format_price(s.total);
            let status = s.status.clone();
            (s.receipt_no.clone(), customer, amount, status)
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search sales..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "report-filters",
                div { class: "form-field",
                    label { "Start Date" }
                    div { class: "input-group input-right",
                        span { class: "input-icon-right", "📅" }
                        input { r#type: "date", value: "{start_date.read()}", oninput: move |e| start_date.set(e.value.clone()) }
                    }
                }
                div { class: "form-field",
                    label { "End Date" }
                    div { class: "input-group input-right",
                        span { class: "input-icon-right", "📅" }
                        input { r#type: "date", value: "{end_date.read()}", oninput: move |e| end_date.set(e.value.clone()) }
                    }
                }
                div { class: "form-field",
                    label { "Status" }
                    select { value: "{status_filter.read()}", onchange: move |e| status_filter.set(e.value.clone()),
                        option { value: "All", "All" }
                        option { value: "paid", "Paid" }
                        option { value: "pending", "Pending" }
                        option { value: "voided", "Voided" }
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Receipt" }
                            th { "Customer" }
                            th { "Amount" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (receipt, customer, amount, status) in rows {
                            tr {
                                td { "{receipt}" }
                                td { "{customer}" }
                                td { "{amount}" }
                                td { StatusChip { label: status } }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SalesTab() -> Element {
    let mut show_sale_modal = use_signal(|| false);
    let mut selected_sale_id = use_signal(|| None as Option<String>);
    let mut sales_state = use_signal(|| {
        let store = get_store_fresh();
        store.sales.clone()
    });
    let mut status_filter = use_signal(|| "all".to_string());
    let mut cashier_filter = use_signal(|| "all".to_string());
    let mut sale_query = use_signal(|| String::new());
    let mut sale_sort = use_signal(|| "date".to_string());
    let mut sale_order = use_signal(|| "desc".to_string());
    let mut sale_page = use_signal(|| 1usize);
    let mut sale_save_msg = use_signal(|| false);
    let mut sale_error = use_signal(|| None::<String>);
    let mut sale_export_msg = use_signal(|| None::<String>);
    let mut sale_export_error = use_signal(|| None::<String>);
    let store = get_store_fresh();
    let sales = sales_state.read().clone();
    let sale_items = extra_list("sale_items.json");
    let scans = &store.scans;
    let is_completed = |status: &str| status.eq_ignore_ascii_case("paid") || status.eq_ignore_ascii_case("completed");
    let completed_sales = sales.iter().filter(|s| is_completed(&s.status)).count();
    let pending_sales = sales.iter().filter(|s| !is_completed(&s.status)).count();
    let mut cashier_list: Vec<String> = sales.iter().map(|s| s.cashier_id.clone()).collect();
    cashier_list.sort();
    cashier_list.dedup();
    let query = sale_query.read().to_lowercase();
    let mut filtered_sales: Vec<crate::data::json_store::SaleRecord> = sales
        .iter()
        .filter(|s| {
            let status_ok = match status_filter.read().as_str() {
                "paid" => is_completed(&s.status),
                "pending" => !is_completed(&s.status),
                "voided" => s.status.eq_ignore_ascii_case("voided"),
                _ => true,
            };
            let cashier_ok = match cashier_filter.read().as_str() {
                "all" => true,
                cashier => s.cashier_id == cashier,
            };
            let query_ok = if query.is_empty() {
                true
            } else {
                let id = s.id.to_lowercase();
                let receipt = s.receipt_no.to_lowercase();
                let cashier = s.cashier_id.to_lowercase();
                id.contains(&query) || receipt.contains(&query) || cashier.contains(&query)
            };
            status_ok && cashier_ok && query_ok
        })
        .cloned()
        .collect();
    let sort_key = sale_sort.read().clone();
    filtered_sales.sort_by(|a, b| {
        match sort_key.as_str() {
            "total" => a.total.partial_cmp(&b.total).unwrap_or(std::cmp::Ordering::Equal),
            "status" => a.status.to_lowercase().cmp(&b.status.to_lowercase()),
            _ => a.created_at.cmp(&b.created_at),
        }
    });
    if sale_order.read().as_str() == "desc" {
        filtered_sales.reverse();
    }
    let page_size = 10usize;
    let total_pages = std::cmp::max(1, (filtered_sales.len() + page_size - 1) / page_size);
    let current_page = (*sale_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_sales.len());
    let page_items: Vec<crate::data::json_store::SaleRecord> = if filtered_sales.is_empty() {
        Vec::new()
    } else {
        filtered_sales[start..end].to_vec()
    };

    let mut sale_rows: Vec<Element> = Vec::new();
    for s in page_items.into_iter() {
        let status_label = if s.status.eq_ignore_ascii_case("voided") {
            "Voided"
        } else if is_completed(&s.status) {
            "Paid"
        } else {
            "Pending"
        };
        let status_class = if s.status.eq_ignore_ascii_case("voided") {
            "data-chip danger"
        } else if is_completed(&s.status) {
            "data-chip"
        } else {
            "data-chip warning"
        };
        let sid = s.id.clone();
        let scashier = s.cashier_id.clone();
        let stotal = s.total;
        let screated = s.created_at.clone();
        let mut selected_sale_id = selected_sale_id.clone();
        let mut show_sale_modal = show_sale_modal.clone();
        let mut sales_state = sales_state.clone();
        let mut sale_error = sale_error.clone();
        let mut sale_save_msg = sale_save_msg.clone();
        let sid_for_view = sid.clone();
        let sid_for_void = sid.clone();
        sale_rows.push(rsx! {
            tr {
                td { "{sid}" }
                td { "{format_datetime(&screated)}" }
                td { "{format_price(stotal)}" }
                td { "{scashier}" }
                td { span { class: "{status_class}", "{status_label}" } }
                td { div { class: "table-actions",
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            selected_sale_id.set(Some(sid_for_view.clone()));
                            show_sale_modal.set(true);
                        },
                        "🔍 View"
                    }
                    button {
                        class: "btn-small btn-danger",
                        onclick: move |_| {
                            let mut updated = sales_state.read().clone();
                            if let Some(sale) = updated.iter_mut().find(|sale| sale.id == sid_for_void) {
                                sale.status = "voided".to_string();
                            }
                            match save_sales_to_json(&updated) {
                                Ok(_) => {
                                    sales_state.set(updated);
                                    sale_error.set(None);
                                    sale_save_msg.set(true);
                                    spawn(async move {
                                        sleep_ms(2_000).await;
                                        sale_save_msg.set(false);
                                    });
                                }
                                Err(err) => sale_error.set(Some(err)),
                            }
                        },
                        "🗑️ Void"
                    }
                } }
            }
        });
    }

    rsx! {
        div { class: "content-card",
            h2 { "💸 Sales / Transactions" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {sales.len()}" }
                div { class: "ops-pill success", "Completed: {completed_sales}" }
                div { class: "ops-pill warning", "Pending: {pending_sales}" }
            }

            if sale_save_msg.read().clone() {
                div { class: "message",
                    "✅ Sale updated successfully!"
                }
            }
            if let Some(err) = sale_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = sale_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = sale_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-summary",
                div { class: "filter-group filter-inline",
                    label { "Status" }
                    select {
                        value: "{status_filter}",
                        onchange: move |e| status_filter.set(e.value()),
                        option { value: "all", "All" }
                        option { value: "paid", "Paid" }
                        option { value: "pending", "Pending" }
                        option { value: "voided", "Voided" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Cashier" }
                    select {
                        value: "{cashier_filter}",
                        onchange: move |e| cashier_filter.set(e.value()),
                        option { value: "all", "All" }
                        { cashier_list.iter().map(|c| rsx!( option { value: "{c}", "{c}" } )) }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Search" }
                    input {
                        placeholder: "Tx ID, receipt, cashier",
                        value: "{sale_query}",
                        oninput: move |e| {
                            sale_query.set(e.value());
                            sale_page.set(1);
                        },
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Sort" }
                    select {
                        value: "{sale_sort}",
                        onchange: move |e| sale_sort.set(e.value()),
                        option { value: "date", "Date" }
                        option { value: "total", "Total" }
                        option { value: "status", "Status" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Order" }
                    select {
                        value: "{sale_order}",
                        onchange: move |e| sale_order.set(e.value()),
                        option { value: "desc", "Desc" }
                        option { value: "asc", "Asc" }
                    }
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        let mut updated = sales_state.read().clone();
                        normalize_sales_statuses(&mut updated);
                        match save_sales_to_json(&updated) {
                            Ok(_) => {
                                sales_state.set(updated);
                                sale_export_error.set(None);
                                sale_export_msg.set(Some("✅ Sales statuses normalized".to_string()));
                            }
                            Err(err) => sale_export_error.set(Some(err)),
                        }
                    },
                    "Normalize Status"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_sales_csv(&sales) {
                            Ok(path) => {
                                sale_export_error.set(None);
                                sale_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => sale_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Tx ID" }
                            th { "Timestamp" }
                            th { "Total" }
                            th { "Cashier" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if sale_rows.is_empty() {
                            tr {
                                td { colspan: "6",
                                    div { class: "empty-state", "No sales yet." }
                                }
                            }
                        } else {
                            { sale_rows.into_iter() }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| sale_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| sale_page.set(current_page + 1),
                    "Next"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Sale Items" }
                        span { class: "card-note", "{sale_items.len()} items" }
                    }
                    div { class: "data-list",
                        if sale_items.is_empty() {
                            div { class: "empty-state", "No sale items logged." }
                        } else {
                            { sale_items.iter().rev().take(4).map(|item| {
                                let title = pick_first(item, &["name", "product_name", "product_id", "id"]);
                                let qty = pick_first(item, &["quantity", "qty", "count"]);
                                let price = pick_first(item, &["price", "amount", "total"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{title}" }
                                            span { "Qty: {qty}" }
                                        }
                                        span { class: "data-chip", "{price}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Scans" }
                        span { class: "card-note", "{scans.len()} scans" }
                    }
                    div { class: "data-list",
                        if scans.is_empty() {
                            div { class: "empty-state", "No scans recorded." }
                        } else {
                            { scans.iter().rev().take(4).map(|scan| {
                                let when = format_datetime(&scan.scanned_at);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "Product {scan.product_id}" }
                                            span { "{when}" }
                                        }
                                        span { class: "data-chip", "{scan.quantity} pcs" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            if show_sale_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "🧾 Sale Details" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_sale_modal.set(false),
                                "✕"
                            }
                        }
                        {
                            let sale_id = selected_sale_id.read().clone().unwrap_or_default();
                            let sale = sales.iter().find(|s| s.id == sale_id);
                            let items: Vec<&serde_json::Value> = sale_items
                                .iter()
                                .copied()
                                .filter(|item| pick_first(item, &["sale_id", "saleId", "sale"]) == sale_id)
                                .collect();
                            rsx!(
                                div { class: "form-group",
                                    if let Some(sale) = sale {
                                        div { class: "data-stats",
                                            div { class: "data-stat",
                                                span { "Receipt" }
                                                strong { "{sale.receipt_no}" }
                                            }
                                            div { class: "data-stat",
                                                span { "Cashier" }
                                                strong { "{sale.cashier_id}" }
                                            }
                                            div { class: "data-stat",
                                                span { "Status" }
                                                strong { "{sale.status}" }
                                            }
                                        }
                                        div { class: "data-stats",
                                            div { class: "data-stat",
                                                span { "Subtotal" }
                                                strong { "{format_price(sale.subtotal)}" }
                                            }
                                            div { class: "data-stat",
                                                span { "Tax" }
                                                strong { "{format_price(sale.tax)}" }
                                            }
                                            div { class: "data-stat",
                                                span { "Total" }
                                                strong { "{format_price(sale.total)}" }
                                            }
                                        }
                                    } else {
                                        div { class: "empty-state", "Sale not found." }
                                    }
                                }

                                div { class: "form-group",
                                    h4 { "Line Items" }
                                    if items.is_empty() {
                                        div { class: "empty-state", "No sale items for this transaction." }
                                    } else {
                                        div { class: "table-container table-compact",
                                            table {
                                                thead {
                                                    tr {
                                                        th { "Item" }
                                                        th { "Qty" }
                                                        th { "Unit" }
                                                        th { "Total" }
                                                    }
                                                }
                                                tbody {
                                                    { items.iter().map(|item| {
                                                        let name = pick_first(item, &["name", "product_name", "product_id", "id"]);
                                                        let qty = value_i32(item, "quantity")
                                                            .or_else(|| value_i32(item, "qty"))
                                                            .unwrap_or(0);
                                                        let unit = value_f32(item, "unit_price")
                                                            .or_else(|| value_f32(item, "price"))
                                                            .unwrap_or(0.0);
                                                        let line = value_f32(item, "line_total")
                                                            .or_else(|| value_f32(item, "total"))
                                                            .unwrap_or(unit * qty as f32);
                                                        rsx!(
                                                            tr {
                                                                td { "{name}" }
                                                                td { "{qty}" }
                                                                td { "{format_price(unit)}" }
                                                                td { "{format_price(line)}" }
                                                            }
                                                        )
                                                    }) }
                                                }
                                            }
                                        }
                                    }
                                }
                            )
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_sale_modal.set(false),
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StaffTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut staff_query = use_signal(|| String::new());
    let mut staff_sort = use_signal(|| "name".to_string());
    let mut staff_order = use_signal(|| "asc".to_string());
    let mut staff_page = use_signal(|| 1usize);
    let mut staff_export_msg = use_signal(|| None::<String>);
    let mut staff_export_error = use_signal(|| None::<String>);
    let mut staff_username = use_signal(|| String::new());
    let mut staff_email = use_signal(|| String::new());
    let mut staff_password = use_signal(|| String::new());
    let mut staff_image = use_signal(|| None::<String>);
    let store = get_store_fresh();
    let users = store.users.clone();
    let sales = &store.sales;
    
    let handle_add_staff = move |_| {
        if staff_username.read().is_empty() || staff_email.read().is_empty() {
            return;
        }
        // In real app, save to database with image in data/images/profiles/
        staff_username.set(String::new());
        staff_email.set(String::new());
        staff_password.set(String::new());
        staff_image.set(None);
        show_add_modal.set(false);
    };
    
    let query = staff_query.read().to_lowercase();
    let mut filtered_staff: Vec<crate::data::json_store::UserRecord> = users
        .clone()
        .into_iter()
        .filter(|u| {
            if query.is_empty() {
                true
            } else {
                let name = u.username.to_lowercase();
                let email = u.email.to_lowercase();
                let role = format!("{:?}", u.role).to_lowercase();
                let status = u.status.to_lowercase();
                name.contains(&query) || email.contains(&query) || role.contains(&query) || status.contains(&query)
            }
        })
        .collect();
    let sort_key = staff_sort.read().clone();
    filtered_staff.sort_by(|a, b| {
        match sort_key.as_str() {
            "email" => a.email.to_lowercase().cmp(&b.email.to_lowercase()),
            "status" => a.status.to_lowercase().cmp(&b.status.to_lowercase()),
            _ => a.username.to_lowercase().cmp(&b.username.to_lowercase()),
        }
    });
    if staff_order.read().as_str() == "desc" {
        filtered_staff.reverse();
    }
    let page_size = 10usize;
    let total_pages = std::cmp::max(1, (filtered_staff.len() + page_size - 1) / page_size);
    let current_page = (*staff_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_staff.len());
    let page_items: Vec<crate::data::json_store::UserRecord> = if filtered_staff.is_empty() {
        Vec::new()
    } else {
        filtered_staff[start..end].to_vec()
    };
    let mut staff_rows: Vec<Element> = Vec::new();
    for u in page_items.into_iter() {
        let status_label = if u.status == "active" { "✅ Active" } else { "⏸️ Inactive" };
        let role_label = match u.role {
            crate::data::models::user::UserRole::Admin => "🛡️ Admin",
            crate::data::models::user::UserRole::Staff => "👤 Staff",
        };
        let initials = {
            let mut chars = u.username.chars().filter(|c| c.is_alphabetic());
            let first = chars.next().unwrap_or('U');
            let second = chars.next().unwrap_or(first);
            format!("{}{}", first, second).to_uppercase()
        };
        let uname = u.username.clone();
        let email = u.email.clone();
        let img = u.profile_image.clone();
        staff_rows.push(rsx!(
            tr {
                td {
                    if let Some(img) = img {
                        img { class: "table-avatar", src: "{img}" }
                    } else {
                        span { class: "table-avatar-fallback", "{initials}" }
                    }
                }
                td { "{uname}" }
                td { "{email}" }
                td { "{role_label}" }
                td { "{status_label}" }
                td {
                    div { class: "table-actions",
                        button { class: "btn-small", "✏️ Edit" }
                        button { class: "btn-small btn-danger", "🗑️ Remove" }
                    }
                }
            }
        ));
    }

    let mut staff_perf: HashMap<String, (i32, f32)> = HashMap::new();
    for s in sales.iter() {
        let entry = staff_perf.entry(s.cashier_id.clone()).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += s.total;
    }
    let mut staff_perf_vec: Vec<(String, i32, f32)> = staff_perf
        .into_iter()
        .map(|(id, (count, total))| (id, count, total))
        .collect();
    staff_perf_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    rsx! {
        div { class: "tab-content",
            div { class: "products-header",
                h2 { "👥 Staff Management" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_add_modal.set(true),
                    "+ Add Staff"
                }
            }

            div { class: "ops-summary",
                div { class: "filter-group filter-inline",
                    label { "Search" }
                    input {
                        placeholder: "Name, email, role, status",
                        value: "{staff_query}",
                        oninput: move |e| {
                            staff_query.set(e.value());
                            staff_page.set(1);
                        },
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Sort" }
                    select {
                        value: "{staff_sort}",
                        onchange: move |e| staff_sort.set(e.value()),
                        option { value: "name", "Name" }
                        option { value: "email", "Email" }
                        option { value: "status", "Status" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Order" }
                    select {
                        value: "{staff_order}",
                        onchange: move |e| staff_order.set(e.value()),
                        option { value: "asc", "Asc" }
                        option { value: "desc", "Desc" }
                    }
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_staff_csv(&users) {
                            Ok(path) => {
                                staff_export_error.set(None);
                                staff_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => staff_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Avatar" }
                            th { "Username" }
                            th { "Email" }
                            th { "Role" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if staff_rows.is_empty() {
                            tr {
                                td { colspan: "6",
                                    div { class: "empty-state", "No staff users found." }
                                }
                            }
                        } else {
                            { staff_rows.into_iter() }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| staff_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| staff_page.set(current_page + 1),
                    "Next"
                }
            }

            if let Some(msg) = staff_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = staff_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Top Staff Performance" }
                        span { class: "card-note", "By sales volume" }
                    }
                    div { class: "data-list",
                        if staff_perf_vec.is_empty() {
                            div { class: "empty-state", "No sales data yet." }
                        } else {
                            { staff_perf_vec.iter().take(4).map(|(id, count, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{id}" }
                                            span { "{count} sales" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            if show_add_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "➕ Add New Staff" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_add_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Username *" }
                            input {
                                placeholder: "e.g., john",
                                value: "{staff_username}",
                                oninput: move |e| staff_username.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Email *" }
                            input {
                                r#type: "email",
                                placeholder: "e.g., john@pos.local",
                                value: "{staff_email}",
                                oninput: move |e| staff_email.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Password *" }
                            input {
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{staff_password}",
                                oninput: move |e| staff_password.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Profile Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                input {
                                    r#type: "file",
                                    accept: "image/*",
                                    onchange: move |_e| {
                                        // In real app, handle file upload to data/images/profiles/
                                    },
                                }
                            }
                            if let Some(img) = staff_image.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_add_staff,
                                "✅ Add Staff"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_add_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn CustomersTab() -> Element {
    let store = get_store_fresh();
    let customers = extra_list("customers.json");
    let mut customers_state = use_signal(|| customers.iter().map(|c| (*c).clone()).collect::<Vec<_>>());
    let customers_owned = customers_state.read().clone();
    let active_customers = customers_owned.iter().filter(|c| {
        let status = pick_first(c, &["status"]);
        status.is_empty() || status.eq_ignore_ascii_case("active")
    }).count();
    let sales = &store.sales;
    let mut customer_export_msg = use_signal(|| None::<String>);
    let mut customer_export_error = use_signal(|| None::<String>);
    let mut customer_query = use_signal(|| String::new());
    let mut customer_sort = use_signal(|| "name".to_string());
    let mut customer_order = use_signal(|| "asc".to_string());
    let mut customer_page = use_signal(|| 1usize);
    let mut add_name = use_signal(|| String::new());
    let mut add_email = use_signal(|| String::new());
    let mut add_phone = use_signal(|| String::new());
    let mut add_status = use_signal(|| "active".to_string());
    let mut add_image_payload = use_signal(|| None::<String>);
    let mut add_image_type = use_signal(|| None::<String>);
    let mut add_image_preview = use_signal(|| None::<String>);
    let mut add_msg = use_signal(|| None::<String>);
    let mut add_error = use_signal(|| None::<String>);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_email = use_signal(|| String::new());
    let mut edit_phone = use_signal(|| String::new());
    let mut edit_status = use_signal(|| "active".to_string());
    let mut edit_image_payload = use_signal(|| None::<String>);
    let mut edit_image_type = use_signal(|| None::<String>);
    let mut edit_image_name = use_signal(|| None::<String>);
    let mut edit_image_preview = use_signal(|| None::<String>);
    let mut edit_msg = use_signal(|| None::<String>);
    let mut edit_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);

    let handle_add_customer = {
        let mut customers_state = customers_state.clone();
        let mut add_name = add_name.clone();
        let mut add_email = add_email.clone();
        let mut add_phone = add_phone.clone();
        let mut add_status = add_status.clone();
        let mut add_image_payload = add_image_payload.clone();
        let mut add_image_type = add_image_type.clone();
        let mut add_image_preview = add_image_preview.clone();
        let mut add_msg = add_msg.clone();
        let mut add_error = add_error.clone();
        move |_| {
            if add_name.read().is_empty() {
                add_error.set(Some("Customer name is required.".to_string()));
                return;
            }
            if !is_valid_email(&add_email.read()) {
                add_error.set(Some("Please enter a valid email address.".to_string()));
                return;
            }
            let mut updated = customers_state.read().clone();
            let id = new_id("cust");
            let mut image_filename: Option<String> = None;
            let mut image_type_saved: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (add_image_payload.read().clone(), add_image_type.read().clone()) {
                match ImageService::save_user_image(&payload, &img_type, &id) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        add_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            updated.push(json!({
                "id": id,
                "name": add_name.read().clone(),
                "email": add_email.read().clone(),
                "phone": add_phone.read().clone(),
                "status": add_status.read().clone(),
                "loyalty_points": 0,
                "profile_image": image_filename,
                "profile_image_type": image_type_saved,
                "created_at": now,
                "updated_at": now,
            }));
            match save_extra_to_json("customers.json", &updated) {
                Ok(_) => {
                    customers_state.set(updated);
                    add_error.set(None);
                    add_msg.set(Some("✅ Customer added.".to_string()));
                    add_name.set(String::new());
                    add_email.set(String::new());
                    add_phone.set(String::new());
                    add_status.set("active".to_string());
                    add_image_payload.set(None);
                    add_image_type.set(None);
                    add_image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_msg.set(None);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };

    let handle_edit_customer = {
        let mut customers_state = customers_state.clone();
        let mut edit_id = edit_id.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_phone = edit_phone.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_image_payload = edit_image_payload.clone();
        let mut edit_image_type = edit_image_type.clone();
        let mut edit_image_name = edit_image_name.clone();
        let mut edit_image_preview = edit_image_preview.clone();
        let mut edit_msg = edit_msg.clone();
        let mut edit_error = edit_error.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        move |_| {
            let id = match edit_id.read().clone() {
                Some(id) => id,
                None => return,
            };
            let mut updated = customers_state.read().clone();
            let mut image_filename = edit_image_name.read().clone();
            let mut image_type_saved = edit_image_type.read().clone();
            if let (Some(payload), Some(img_type)) = (edit_image_payload.read().clone(), edit_image_type.read().clone()) {
                match ImageService::save_user_image(&payload, &img_type, &id) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        edit_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            if let Some(entry) = updated.iter_mut().find(|c| pick_first(c, &["id", "customer_id"]) == id) {
                if let Some(obj) = entry.as_object_mut() {
                    if !is_valid_email(&edit_email.read()) {
                        edit_error.set(Some("Please enter a valid email address.".to_string()));
                        return;
                    }
                    obj.insert("name".to_string(), json!(edit_name.read().clone()));
                    obj.insert("email".to_string(), json!(edit_email.read().clone()));
                    obj.insert("phone".to_string(), json!(edit_phone.read().clone()));
                    obj.insert("status".to_string(), json!(edit_status.read().clone()));
                    obj.insert("profile_image".to_string(), json!(image_filename));
                    obj.insert("profile_image_type".to_string(), json!(image_type_saved));
                    obj.insert("updated_at".to_string(), json!(now_iso()));
                }
            } else {
                edit_error.set(Some("Customer not found.".to_string()));
                return;
            }
            match save_extra_to_json("customers.json", &updated) {
                Ok(_) => {
                    customers_state.set(updated);
                    edit_error.set(None);
                    edit_msg.set(Some("✅ Customer updated.".to_string()));
                    show_edit_modal.set(false);
                    edit_id.set(None);
                    edit_image_payload.set(None);
                    edit_image_type.set(None);
                    edit_image_name.set(None);
                    edit_image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        edit_msg.set(None);
                    });
                }
                Err(err) => edit_error.set(Some(err)),
            }
        }
    };

    let query = customer_query.read().to_lowercase();
    let mut filtered_customers: Vec<serde_json::Value> = customers_owned
        .clone()
        .into_iter()
        .filter(|c| {
            if query.is_empty() {
                true
            } else {
                let name = pick_first(c, &["name", "full_name", "username", "id"]).to_lowercase();
                let email = pick_first(c, &["email", "contact_email", "phone"]).to_lowercase();
                let tier = pick_first(c, &["tier", "segment", "status"]).to_lowercase();
                name.contains(&query) || email.contains(&query) || tier.contains(&query)
            }
        })
        .collect();
    let sort_key = customer_sort.read().clone();
    filtered_customers.sort_by(|a, b| {
        let a_val = pick_first(a, &["name", "full_name", "username", "id"]).to_lowercase();
        let b_val = pick_first(b, &["name", "full_name", "username", "id"]).to_lowercase();
        match sort_key.as_str() {
            "status" => pick_first(a, &["status", "tier", "segment"]).to_lowercase()
                .cmp(&pick_first(b, &["status", "tier", "segment"]).to_lowercase()),
            _ => a_val.cmp(&b_val),
        }
    });
    if customer_order.read().as_str() == "desc" {
        filtered_customers.reverse();
    }
    let page_size = 8usize;
    let total_pages = std::cmp::max(1, (filtered_customers.len() + page_size - 1) / page_size);
    let current_page = (*customer_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_customers.len());
    let page_items: Vec<serde_json::Value> = if filtered_customers.is_empty() {
        Vec::new()
    } else {
        filtered_customers[start..end].to_vec()
    };
    let mut customer_rows: Vec<Element> = Vec::new();
    for c in page_items.clone().into_iter() {
        let name = pick_first(&c, &["name", "full_name", "username", "id"]);
        let email = pick_first(&c, &["email", "contact_email", "phone"]);
        let tier = pick_first(&c, &["tier", "segment", "status"]);
        customer_rows.push(rsx!(
            div { class: "data-row",
                div { class: "data-main",
                    strong { "{name}" }
                    span { "{email}" }
                }
                span { class: "data-chip", "{tier}" }
            }
        ));
    }
    let mut customer_sales: HashMap<String, i32> = HashMap::new();
    for s in sales.iter() {
        if let Some(cid) = s.customer_id.clone() {
            *customer_sales.entry(cid).or_insert(0) += 1;
        }
    }
    let repeat_customers = customer_sales.values().filter(|c| **c > 1).count();
    let unique_customers = customer_sales.len();
    let repeat_rate = if unique_customers == 0 {
        0.0
    } else {
        (repeat_customers as f32 / unique_customers as f32) * 100.0
    };
    let repeat_rate_s = format!("{:.1}%", repeat_rate);

    rsx! {
        div { class: "content-card",
            h2 { "🧑‍🤝‍🧑 Customers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {customers_owned.len()}" }
                div { class: "ops-pill success", "Active: {active_customers}" }
            }

            if let Some(msg) = add_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = add_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = edit_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = edit_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = delete_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = delete_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Add Customer" }
                    span { class: "card-subtitle", "Create a new customer profile." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Customer Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            input {
                                placeholder: "Jane Doe",
                                value: "{add_name}",
                                oninput: move |e| add_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "jane@example.com",
                                value: "{add_email}",
                                oninput: move |e| add_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0200",
                                value: "{add_phone}",
                                oninput: move |e| add_phone.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Status" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{add_status}",
                                onchange: move |e| add_status.set(e.value()),
                                option { value: "active", "Active" }
                                option { value: "inactive", "Inactive" }
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Profile Image" }
                        div { class: "file-input-wrapper",
                            button { class: "btn btn-secondary", "📷 Choose Image" }
                            { let mut add_image_payload = add_image_payload.clone();
                              let mut add_image_type = add_image_type.clone();
                              let mut add_image_preview = add_image_preview.clone();
                              let mut add_error = add_error.clone();
                              rsx!(
                                input {
                                    r#type: "file",
                                    accept: "image/*",
                                    onchange: move |e| {
                                        let path = e.value();
                                        if path.is_empty() {
                                            return;
                                        }
                                        match load_image_from_path(&path) {
                                            Ok((payload, img_type, preview)) => {
                                                add_image_payload.set(Some(payload));
                                                add_image_type.set(Some(img_type));
                                                add_image_preview.set(Some(preview));
                                            }
                                            Err(err) => add_error.set(Some(format!("Image load failed: {}", err))),
                                        }
                                    },
                                }
                              )
                            }
                        }
                        if let Some(img) = add_image_preview.read().clone() {
                            div { class: "image-preview",
                                img { src: "{img}" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_add_customer,
                        "Add Customer"
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "filter-group filter-inline",
                    label { "Search" }
                    input {
                        placeholder: "Name, email, segment",
                        value: "{customer_query}",
                        oninput: move |e| {
                            customer_query.set(e.value());
                            customer_page.set(1);
                        },
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Sort" }
                    select {
                        value: "{customer_sort}",
                        onchange: move |e| customer_sort.set(e.value()),
                        option { value: "name", "Name" }
                        option { value: "status", "Status" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Order" }
                    select {
                        value: "{customer_order}",
                        onchange: move |e| customer_order.set(e.value()),
                        option { value: "asc", "Asc" }
                        option { value: "desc", "Desc" }
                    }
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_customers_csv(&customers_owned) {
                            Ok(path) => {
                                customer_export_error.set(None);
                                customer_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => customer_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
            }

            div { class: "data-ops-grid",
                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Recent Customers" }
                        span { class: "card-note", "{customers_owned.len()} records" }
                    }
                    div { class: "data-list",
                        if customer_rows.is_empty() {
                            div { class: "empty-state", "No customer data found." }
                        } else {
                            { customer_rows.into_iter() }
                        }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Customer Insights" }
                        span { class: "card-note", "Engagement" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Active" }
                            strong { "{active_customers}" }
                        }
                        div { class: "data-stat",
                            span { "Inactive" }
                            strong { "{customers_owned.len().saturating_sub(active_customers)}" }
                        }
                        div { class: "data-stat",
                            span { "Records" }
                            strong { "{customers_owned.len()}" }
                        }
                    }
                    div { class: "data-list",
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Unique customers" }
                                span { "From sales history" }
                            }
                            span { class: "data-chip", "{unique_customers}" }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Repeat customers" }
                                span { "2+ purchases" }
                            }
                            span { class: "data-chip", "{repeat_customers}" }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Repeat rate" }
                                span { "Repeat / unique" }
                            }
                            span { class: "data-chip", "{repeat_rate_s}" }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Customer Directory" }
                    span { class: "card-subtitle", "Manage customer records." }
                }
                div { class: "admin-card-body",
                    div { class: "table-container",
                        table { class: "admin-table",
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Email" }
                                    th { "Phone" }
                                    th { "Status" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                if filtered_customers.is_empty() {
                                    tr { td { colspan: "5",
                                        div { class: "empty-state", "No customers match your filters." }
                                    } }
                                } else {
                                    { filtered_customers.iter().map(|c| {
                                        let cid = pick_first(c, &["id", "customer_id"]);
                                        let name = pick_first(c, &["name", "full_name", "username", "id"]);
                                        let email = pick_first(c, &["email", "contact_email", "phone"]);
                                        let phone = pick_first(c, &["phone", "contact_phone", "mobile"]);
                                        let status = pick_first(c, &["status", "tier", "segment"]);
                                        let image_name = pick_first(c, &["profile_image", "image"]);
                                        let image_type = pick_first(c, &["profile_image_type", "image_type"]);
                                        let mut show_edit_modal = show_edit_modal.clone();
                                        let mut edit_id = edit_id.clone();
                                        let mut edit_name = edit_name.clone();
                                        let mut edit_email = edit_email.clone();
                                        let mut edit_phone = edit_phone.clone();
                                        let mut edit_status = edit_status.clone();
                                        let mut edit_image_name = edit_image_name.clone();
                                        let mut edit_image_type = edit_image_type.clone();
                                        let mut edit_image_payload = edit_image_payload.clone();
                                        let mut edit_image_preview = edit_image_preview.clone();
                                        let mut customers_state = customers_state.clone();
                                        let mut delete_msg = delete_msg.clone();
                                        let mut delete_error = delete_error.clone();
                                        let cid_for_delete = cid.clone();
                                        rsx!(
                                            tr {
                                                td { "{name}" }
                                                td { "{email}" }
                                                td { "{phone}" }
                                                td { span { class: "data-chip", "{status}" } }
                                                td { div { class: "table-actions",
                                                    button {
                                                        class: "btn-small",
                                                        onclick: move |_| {
                                                            edit_id.set(Some(cid.clone()));
                                                            edit_name.set(name.clone());
                                                            edit_email.set(email.clone());
                                                            edit_phone.set(phone.clone());
                                                            edit_status.set(if status.is_empty() { "active".to_string() } else { status.clone() });
                                                            let image_name_opt = if image_name.is_empty() { None } else { Some(image_name.clone()) };
                                                            let image_type_opt = if image_type.is_empty() { None } else { Some(image_type.clone()) };
                                                            edit_image_name.set(image_name_opt.clone());
                                                            edit_image_type.set(image_type_opt.clone());
                                                            edit_image_payload.set(None);
                                                            let preview = image_name_opt
                                                                .as_deref()
                                                                .and_then(|name| image_preview_from_filename(name, &image_type_opt, PROFILES_IMAGES_DIR));
                                                            edit_image_preview.set(preview);
                                                            show_edit_modal.set(true);
                                                        },
                                                        "✏️ Edit"
                                                    }
                                                    button {
                                                        class: "btn-small btn-danger",
                                                        onclick: move |_| {
                                                            let mut updated = customers_state.read().clone();
                                                            updated.retain(|c| pick_first(c, &["id", "customer_id"]) != cid_for_delete);
                                                            match save_extra_to_json("customers.json", &updated) {
                                                                Ok(_) => {
                                                                    customers_state.set(updated);
                                                                    delete_error.set(None);
                                                                    delete_msg.set(Some("✅ Customer deleted.".to_string()));
                                                                    spawn(async move {
                                                                        sleep_ms(2_000).await;
                                                                        delete_msg.set(None);
                                                                    });
                                                                }
                                                                Err(err) => delete_error.set(Some(err)),
                                                            }
                                                        },
                                                        "🗑️ Delete"
                                                    }
                                                } }
                                            }
                                        )
                                    }) }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| customer_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| customer_page.set(current_page + 1),
                    "Next"
                }
            }
            if let Some(msg) = customer_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = customer_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Customer" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }
                        div { class: "form-group",
                            label { "Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    value: "{edit_name}",
                                    oninput: move |e| edit_name.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input {
                                    value: "{edit_email}",
                                    oninput: move |e| edit_email.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input {
                                    value: "{edit_phone}",
                                    oninput: move |e| edit_phone.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select {
                                    value: "{edit_status}",
                                    onchange: move |e| edit_status.set(e.value()),
                                    option { value: "active", "Active" }
                                    option { value: "inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Profile Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut edit_image_payload = edit_image_payload.clone();
                                  let mut edit_image_type = edit_image_type.clone();
                                  let mut edit_image_preview = edit_image_preview.clone();
                                  let mut edit_error = edit_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    edit_image_payload.set(Some(payload));
                                                    edit_image_type.set(Some(img_type));
                                                    edit_image_preview.set(Some(preview));
                                                }
                                                Err(err) => edit_error.set(Some(format!("Image load failed: {}", err))),
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = edit_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_customer,
                                "Save Changes"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(false),
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InventoryTab() -> Element {
    let store = get_store_fresh();
    let products = &store.products;
    let movements = extra_list("inventory_movements.json");
    let categories = extra_list("categories.json");
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock = products.iter().filter(|p| p.quantity < LOW_STOCK_THRESHOLD).count();
    let total_value: f32 = products.iter().map(|p| p.price * p.quantity as f32).sum();
    let mut category_values: HashMap<String, f32> = HashMap::new();
    for p in products.iter() {
        *category_values.entry(p.category.clone()).or_insert(0.0) += p.price * p.quantity as f32;
    }
    let mut category_value_vec: Vec<(String, f32)> = category_values.into_iter().collect();
    category_value_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut reorder_list: Vec<&crate::data::models::product::Product> = products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .collect();
    reorder_list.sort_by(|a, b| a.quantity.cmp(&b.quantity));

    rsx! {
        div { class: "content-card",
            h2 { "🗄️ Inventory" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Products: {products.len()}" }
                div { class: "ops-pill warning", "Threshold: {LOW_STOCK_THRESHOLD}" }
                div { class: "ops-pill warning", "Low stock: {low_stock}" }
                div { class: "ops-pill danger", "Out: {out_of_stock}" }
                div { class: "ops-pill", "Movements: {movements.len()}" }
                div { class: "ops-pill", "Value: {format_price(total_value)}" }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Movements" }
                        span { class: "card-note", "{movements.len()} logs" }
                    }
                    div { class: "data-list",
                        if movements.is_empty() {
                            div { class: "empty-state", "No inventory movements yet." }
                        } else {
                            { movements.iter().rev().take(5).map(|m| {
                                let item = pick_first(m, &["product_name", "product_id", "sku", "id"]);
                                let qty = pick_first(m, &["quantity", "qty", "delta"]);
                                let kind = pick_first(m, &["type", "reason", "action"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{item}" }
                                            span { "{kind}" }
                                        }
                                        span { class: "data-chip", "{qty}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Categories" }
                        span { class: "card-note", "{categories.len()} groups" }
                    }
                    div { class: "data-list",
                        if categories.is_empty() {
                            div { class: "empty-state", "No categories configured." }
                        } else {
                            { categories.iter().take(6).map(|c| {
                                let name = pick_first(c, &["name", "title", "category"]);
                                let code = pick_first(c, &["code", "slug", "id"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "Code: {code}" }
                                        }
                                        span { class: "data-chip", "Active" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Category Valuation" }
                        span { class: "card-note", "{category_value_vec.len()} categories" }
                    }
                    div { class: "data-list",
                        if category_value_vec.is_empty() {
                            div { class: "empty-state", "No valuation data yet." }
                        } else {
                            { category_value_vec.iter().take(6).map(|(name, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "Inventory value" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Reorder Insights" }
                        span { class: "card-note", "{reorder_list.len()} items" }
                    }
                    div { class: "data-list",
                        if reorder_list.is_empty() {
                            div { class: "empty-state", "No low-stock items." }
                        } else {
                            { reorder_list.iter().take(6).map(|p| {
                                let status = if p.quantity == 0 { "Out" } else { "Low" };
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{p.name}" }
                                            span { "{p.category}" }
                                        }
                                        span { class: "data-chip", "{status}: {p.quantity}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PaymentsTab() -> Element {
    let store = get_store_fresh();
    let payments = extra_list("payments.json");
    let payments_owned: Vec<serde_json::Value> = payments.iter().map(|p| (*p).clone()).collect();
    let payments_for_normalize = payments_owned.clone();
    let payments_for_summary = payments_owned.clone();
    let payments_for_ledger = payments_owned.clone();
    let mut export_msg = use_signal(|| None::<String>);
    let mut export_error = use_signal(|| None::<String>);
    let paid = payments
        .iter()
        .filter(|p| value_is(p, "status", "paid") || p.get("status").is_none())
        .count();
    let pending = payments.iter().filter(|p| value_is(p, "status", "pending")).count();
    let failed = payments.iter().filter(|p| value_is(p, "status", "failed")).count();
    let total_amount: f32 = payments
        .iter()
        .map(|p| value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0))
        .sum();
    let mut method_totals: HashMap<String, (i32, f32)> = HashMap::new();
    let mut daily_totals: HashMap<String, f32> = HashMap::new();
    for p in payments.iter() {
        let method = pick_first(p, &["method", "type", "channel"]);
        let amount = value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0);
        let entry = method_totals.entry(method).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += amount;
        let day = pick_first(p, &["paid_at", "created_at", "date"]).split('T').next().unwrap_or("—").to_string();
        *daily_totals.entry(day).or_insert(0.0) += amount;
    }
    let mut method_vec: Vec<(String, i32, f32)> = method_totals
        .into_iter()
        .map(|(m, (count, total))| (m, count, total))
        .collect();
    method_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    let mut daily_vec: Vec<(String, f32)> = daily_totals.into_iter().collect();
    daily_vec.sort_by(|a, b| a.0.cmp(&b.0));

    rsx! {
        div { class: "content-card",
            h2 { "💳 Payments" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {payments.len()}" }
                div { class: "ops-pill success", "Paid: {paid}" }
                div { class: "ops-pill warning", "Pending: {pending}" }
                div { class: "ops-pill danger", "Failed: {failed}" }
                div { class: "ops-pill", "Volume: {format_price(total_amount)}" }
            }

            if let Some(msg) = export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Payments" }
                        span { class: "card-note", "{payments.len()} records" }
                    }
                    div { class: "data-list",
                        if payments.is_empty() {
                            div { class: "empty-state", "No payment records available." }
                        } else {
                            { payments.iter().rev().take(6).map(|p| {
                                let method = pick_first(p, &["method", "type", "channel"]);
                                let amount = value_f32(p, "amount").or_else(|| value_f32(p, "total")).unwrap_or(0.0);
                                let status = pick_first(p, &["status", "state", "paid_at"]);
                                let reference = pick_first(p, &["reference", "ref", "id"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{method} · {reference}" }
                                            span { "{status}" }
                                        }
                                        span { class: "data-chip", "{format_price(amount)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Payment Breakdown" }
                        span { class: "card-note", "{method_vec.len()} methods" }
                    }
                    div { class: "data-list",
                        if method_vec.is_empty() {
                            div { class: "empty-state", "No payment methods yet." }
                        } else {
                            { method_vec.iter().map(|(method, count, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{method}" }
                                            span { "{count} payments" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Daily Totals" }
                        span { class: "card-note", "{daily_vec.len()} days" }
                    }
                    div { class: "data-list",
                        if daily_vec.is_empty() {
                            div { class: "empty-state", "No daily totals yet." }
                        } else {
                            { daily_vec.iter().rev().take(6).map(|(day, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{day}" }
                                            span { "Settled volume" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Reconciliation" }
                        span { class: "card-note", "Exports" }
                    }
                    div { class: "data-list",
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Normalize Status" }
                                span { "Set missing status to paid" }
                            }
                            button {
                                class: "btn-small",
                                onclick: move |_| {
                                    let mut normalized = payments_for_normalize.clone();
                                    for p in normalized.iter_mut() {
                                        if p.get("status").is_none() {
                                            p["status"] = serde_json::Value::String("paid".to_string());
                                        } else if let Some(s) = p.get("status").and_then(|v| v.as_str()) {
                                            if s.eq_ignore_ascii_case("completed") {
                                                p["status"] = serde_json::Value::String("paid".to_string());
                                            }
                                        }
                                    }
                                    match save_payments_to_json(&normalized) {
                                        Ok(_) => {
                                            export_error.set(None);
                                            export_msg.set(Some("✅ Payment statuses normalized".to_string()));
                                        }
                                        Err(err) => export_error.set(Some(err)),
                                    }
                                },
                                "Normalize"
                            }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Settlement Summary" }
                                span { "CSV export for accounting" }
                            }
                            button {
                                class: "btn-small",
                                onclick: move |_| {
                                    match export_payments_summary(&payments_for_summary) {
                                        Ok(path) => {
                                            export_error.set(None);
                                            export_msg.set(Some(format!("✅ Exported to {}", path)));
                                        }
                                        Err(err) => export_error.set(Some(err)),
                                    }
                                },
                                "⬇️ Export CSV"
                            }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Payment Ledger" }
                                span { "Full payment log (last 30 days)" }
                            }
                            button {
                                class: "btn-small",
                                onclick: move |_| {
                                    match export_payments_ledger(&payments_for_ledger) {
                                        Ok(path) => {
                                            export_error.set(None);
                                            export_msg.set(Some(format!("✅ Exported to {}", path)));
                                        }
                                        Err(err) => export_error.set(Some(err)),
                                    }
                                },
                                "⬇️ Export Ledger"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccessTab() -> Element {
    let store = get_store_fresh();
    let roles = extra_list("roles.json");
    let permissions = extra_list("permissions.json");
    let role_permissions = extra_list("role_permissions.json");
    let user_roles = extra_list("user_roles.json");
    let user_options: Vec<(String, String)> = store
        .users
        .iter()
        .map(|u| (u.id.clone(), format!("{} ({})", u.username, u.id)))
        .collect();
    let role_options: Vec<String> = roles
        .iter()
        .map(|r| pick_first(r, &["name", "title", "role"]))
        .filter(|r| r != "—")
        .collect();
    let perm_options: Vec<String> = permissions
        .iter()
        .map(|p| pick_first(p, &["name", "code", "permission"]))
        .filter(|p| p != "—")
        .collect();
    let mut show_role_modal = use_signal(|| false);
    let mut show_permission_modal = use_signal(|| false);
    let mut show_role_link_modal = use_signal(|| false);
    let mut show_user_role_modal = use_signal(|| false);
    let mut role_name = use_signal(|| String::new());
    let mut role_scope = use_signal(|| String::new());
    let mut perm_name = use_signal(|| String::new());
    let mut perm_group = use_signal(|| String::new());
    let mut link_role = use_signal(|| String::new());
    let mut link_permission = use_signal(|| String::new());
    let mut assign_user = use_signal(|| String::new());
    let mut assign_role = use_signal(|| String::new());
    let mut access_msg = use_signal(|| None::<String>);
    let mut access_error = use_signal(|| None::<String>);
    let mut roles_owned: Vec<serde_json::Value> = roles.iter().map(|r| (*r).clone()).collect();
    let mut permissions_owned: Vec<serde_json::Value> = permissions.iter().map(|p| (*p).clone()).collect();
    let mut role_permissions_owned: Vec<serde_json::Value> = role_permissions.iter().map(|rp| (*rp).clone()).collect();
    let mut user_roles_owned: Vec<serde_json::Value> = user_roles.iter().map(|ur| (*ur).clone()).collect();

    rsx! {
        div { class: "content-card",
            h2 { "🛡️ Access Control" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Roles: {roles.len()}" }
                div { class: "ops-pill", "Permissions: {permissions.len()}" }
                div { class: "ops-pill", "Role links: {role_permissions.len()}" }
                div { class: "ops-pill", "User links: {user_roles.len()}" }
                button {
                    class: "btn-small",
                    onclick: move |_| show_role_modal.set(true),
                    "+ Role"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_permission_modal.set(true),
                    "+ Permission"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_role_link_modal.set(true),
                    "+ Role Link"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_user_role_modal.set(true),
                    "+ User Role"
                }
            }

            if let Some(msg) = access_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = access_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Roles" }
                        span { class: "card-note", "{roles.len()} roles" }
                    }
                    div { class: "data-list",
                        if roles.is_empty() {
                            div { class: "empty-state", "No roles configured." }
                        } else {
                            { roles.iter().take(6).map(|r| {
                                let name = pick_first(r, &["name", "title", "role"]);
                                let scope = pick_first(r, &["scope", "level", "description"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{scope}" }
                                        }
                                        span { class: "data-chip", "Role" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Permissions" }
                        span { class: "card-note", "{permissions.len()} permissions" }
                    }
                    div { class: "data-list",
                        if permissions.is_empty() {
                            div { class: "empty-state", "No permissions configured." }
                        } else {
                            { permissions.iter().take(6).map(|p| {
                                let name = pick_first(p, &["name", "code", "permission"]);
                                let group = pick_first(p, &["group", "module", "resource"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{group}" }
                                        }
                                        span { class: "data-chip", "Permission" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Role Assignments" }
                        span { class: "card-note", "{role_permissions.len()} links" }
                    }
                    div { class: "data-list",
                        if role_permissions.is_empty() {
                            div { class: "empty-state", "No role-permission links found." }
                        } else {
                            { role_permissions.iter().take(6).map(|rp| {
                                let role = pick_first(rp, &["role", "role_id", "role_name"]);
                                let perm = pick_first(rp, &["permission", "permission_id", "permission_name"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{role}" }
                                            span { "{perm}" }
                                        }
                                        span { class: "data-chip", "Linked" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "User Roles" }
                        span { class: "card-note", "{user_roles.len()} links" }
                    }
                    div { class: "data-list",
                        if user_roles.is_empty() {
                            div { class: "empty-state", "No user-role assignments found." }
                        } else {
                            { user_roles.iter().take(6).map(|ur| {
                                let user_id = pick_first(ur, &["user_id", "user", "username"]);
                                let role = pick_first(ur, &["role_id", "role", "role_name"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{user_id}" }
                                            span { "{role}" }
                                        }
                                        span { class: "data-chip", "Assigned" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            if show_role_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Add Role" }
                            button { class: "modal-close", onclick: move |_| show_role_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Role Name" }
                            input { value: "{role_name}", oninput: move |e| role_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { "Scope" }
                            input { value: "{role_scope}", oninput: move |e| role_scope.set(e.value()) }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    roles_owned.push(json!({"name": role_name.read().clone(), "scope": role_scope.read().clone()}));
                                    match save_extra_to_json("roles.json", &roles_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Role added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_role_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_role_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_permission_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Add Permission" }
                            button { class: "modal-close", onclick: move |_| show_permission_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Permission Name" }
                            input { value: "{perm_name}", oninput: move |e| perm_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { "Group" }
                            input { value: "{perm_group}", oninput: move |e| perm_group.set(e.value()) }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    permissions_owned.push(json!({"name": perm_name.read().clone(), "group": perm_group.read().clone()}));
                                    match save_extra_to_json("permissions.json", &permissions_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Permission added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_permission_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_permission_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_role_link_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Link Role Permission" }
                            button { class: "modal-close", onclick: move |_| show_role_link_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Role" }
                            if role_options.is_empty() {
                                input { value: "{link_role}", oninput: move |e| link_role.set(e.value()) }
                            } else {
                                select {
                                    value: "{link_role}",
                                    onchange: move |e| link_role.set(e.value()),
                                    option { value: "", "Select role" }
                                    { role_options.iter().map(|r| rsx!( option { value: "{r}", "{r}" } )) }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Permission" }
                            if perm_options.is_empty() {
                                input { value: "{link_permission}", oninput: move |e| link_permission.set(e.value()) }
                            } else {
                                select {
                                    value: "{link_permission}",
                                    onchange: move |e| link_permission.set(e.value()),
                                    option { value: "", "Select permission" }
                                    { perm_options.iter().map(|p| rsx!( option { value: "{p}", "{p}" } )) }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    role_permissions_owned.push(json!({"role": link_role.read().clone(), "permission": link_permission.read().clone()}));
                                    match save_extra_to_json("role_permissions.json", &role_permissions_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Role link added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_role_link_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_role_link_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_user_role_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Assign User Role" }
                            button { class: "modal-close", onclick: move |_| show_user_role_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "User ID" }
                            if user_options.is_empty() {
                                input { value: "{assign_user}", oninput: move |e| assign_user.set(e.value()) }
                            } else {
                                select {
                                    value: "{assign_user}",
                                    onchange: move |e| assign_user.set(e.value()),
                                    option { value: "", "Select user" }
                                    { user_options.iter().map(|(id, label)| rsx!( option { value: "{id}", "{label}" } )) }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Role" }
                            if role_options.is_empty() {
                                input { value: "{assign_role}", oninput: move |e| assign_role.set(e.value()) }
                            } else {
                                select {
                                    value: "{assign_role}",
                                    onchange: move |e| assign_role.set(e.value()),
                                    option { value: "", "Select role" }
                                    { role_options.iter().map(|r| rsx!( option { value: "{r}", "{r}" } )) }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    user_roles_owned.push(json!({"user_id": assign_user.read().clone(), "role": assign_role.read().clone()}));
                                    match save_extra_to_json("user_roles.json", &user_roles_owned) {
                                        Ok(_) => access_msg.set(Some("✅ User role assigned".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_user_role_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_user_role_modal.set(false), "Cancel" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SuppliersTab() -> Element {
    let store = get_store_fresh();
    let suppliers = extra_list("suppliers.json");
    let mut suppliers_state = use_signal(|| suppliers.iter().map(|s| (*s).clone()).collect::<Vec<_>>());
    let suppliers_owned = suppliers_state.read().clone();
    let active_suppliers = suppliers_owned.iter().filter(|s| {
        let status = pick_first(s, &["status"]);
        status.is_empty() || status.eq_ignore_ascii_case("active")
    }).count();
    let mut add_name = use_signal(|| String::new());
    let mut add_contact = use_signal(|| String::new());
    let mut add_phone = use_signal(|| String::new());
    let mut add_email = use_signal(|| String::new());
    let mut add_address = use_signal(|| String::new());
    let mut add_status = use_signal(|| "active".to_string());
    let mut add_msg = use_signal(|| None::<String>);
    let mut add_error = use_signal(|| None::<String>);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_contact = use_signal(|| String::new());
    let mut edit_phone = use_signal(|| String::new());
    let mut edit_email = use_signal(|| String::new());
    let mut edit_address = use_signal(|| String::new());
    let mut edit_status = use_signal(|| "active".to_string());
    let mut edit_msg = use_signal(|| None::<String>);
    let mut edit_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);

    let handle_add_supplier = {
        let mut suppliers_state = suppliers_state.clone();
        let mut add_name = add_name.clone();
        let mut add_contact = add_contact.clone();
        let mut add_phone = add_phone.clone();
        let mut add_email = add_email.clone();
        let mut add_address = add_address.clone();
        let mut add_status = add_status.clone();
        let mut add_msg = add_msg.clone();
        let mut add_error = add_error.clone();
        move |_| {
            if add_name.read().is_empty() {
                add_error.set(Some("Supplier name is required.".to_string()));
                return;
            }
            if !is_valid_email(&add_email.read()) {
                add_error.set(Some("Please enter a valid email address.".to_string()));
                return;
            }
            let mut updated = suppliers_state.read().clone();
            let now = now_iso();
            updated.push(json!({
                "id": new_id("sup"),
                "name": add_name.read().clone(),
                "contact_name": add_contact.read().clone(),
                "phone": add_phone.read().clone(),
                "email": add_email.read().clone(),
                "address": add_address.read().clone(),
                "status": add_status.read().clone(),
                "created_at": now,
                "updated_at": now,
            }));
            match save_extra_to_json("suppliers.json", &updated) {
                Ok(_) => {
                    suppliers_state.set(updated);
                    add_error.set(None);
                    add_msg.set(Some("✅ Supplier added.".to_string()));
                    add_name.set(String::new());
                    add_contact.set(String::new());
                    add_phone.set(String::new());
                    add_email.set(String::new());
                    add_address.set(String::new());
                    add_status.set("active".to_string());
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_msg.set(None);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };

    let handle_edit_supplier = {
        let mut suppliers_state = suppliers_state.clone();
        let mut edit_id = edit_id.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_contact = edit_contact.clone();
        let mut edit_phone = edit_phone.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_address = edit_address.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_msg = edit_msg.clone();
        let mut edit_error = edit_error.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        move |_| {
            let id = match edit_id.read().clone() {
                Some(id) => id,
                None => return,
            };
            let mut updated = suppliers_state.read().clone();
            if let Some(entry) = updated.iter_mut().find(|s| pick_first(s, &["id", "supplier_id"]) == id) {
                if let Some(obj) = entry.as_object_mut() {
                    if !is_valid_email(&edit_email.read()) {
                        edit_error.set(Some("Please enter a valid email address.".to_string()));
                        return;
                    }
                    obj.insert("name".to_string(), json!(edit_name.read().clone()));
                    obj.insert("contact_name".to_string(), json!(edit_contact.read().clone()));
                    obj.insert("phone".to_string(), json!(edit_phone.read().clone()));
                    obj.insert("email".to_string(), json!(edit_email.read().clone()));
                    obj.insert("address".to_string(), json!(edit_address.read().clone()));
                    obj.insert("status".to_string(), json!(edit_status.read().clone()));
                    obj.insert("updated_at".to_string(), json!(now_iso()));
                }
            } else {
                edit_error.set(Some("Supplier not found.".to_string()));
                return;
            }
            match save_extra_to_json("suppliers.json", &updated) {
                Ok(_) => {
                    suppliers_state.set(updated);
                    edit_error.set(None);
                    edit_msg.set(Some("✅ Supplier updated.".to_string()));
                    show_edit_modal.set(false);
                    edit_id.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        edit_msg.set(None);
                    });
                }
                Err(err) => edit_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "content-card",
            h2 { "🚚 Suppliers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {suppliers_owned.len()}" }
                div { class: "ops-pill success", "Active: {active_suppliers}" }
            }

            if let Some(msg) = add_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = add_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = edit_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = edit_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = delete_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = delete_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Add Supplier" }
                    span { class: "card-subtitle", "Create a new supplier profile." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Supplier Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏭" }
                            input {
                                placeholder: "Fresh Farms Ltd",
                                value: "{add_name}",
                                oninput: move |e| add_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Contact Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "👤" }
                            input {
                                placeholder: "A. Rivera",
                                value: "{add_contact}",
                                oninput: move |e| add_contact.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0101",
                                value: "{add_phone}",
                                oninput: move |e| add_phone.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "orders@freshfarms.local",
                                value: "{add_email}",
                                oninput: move |e| add_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Address" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📍" }
                            input {
                                placeholder: "12 Market St",
                                value: "{add_address}",
                                oninput: move |e| add_address.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Status" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{add_status}",
                                onchange: move |e| add_status.set(e.value()),
                                option { value: "active", "Active" }
                                option { value: "inactive", "Inactive" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_add_supplier,
                        "Add Supplier"
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Supplier Directory" }
                    span { class: "card-subtitle", "{suppliers_owned.len()} records" }
                }
                div { class: "admin-card-body",
                    div { class: "table-container",
                        table { class: "admin-table",
                            thead {
                                tr {
                                    th { "Supplier" }
                                    th { "Contact" }
                                    th { "Phone" }
                                    th { "Email" }
                                    th { "Status" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                if suppliers_owned.is_empty() {
                                    tr { td { colspan: "6",
                                        div { class: "empty-state", "No suppliers available." }
                                    } }
                                } else {
                                    { suppliers_owned.iter().map(|s| {
                                        let sid = pick_first(s, &["id", "supplier_id"]);
                                        let name = pick_first(s, &["name", "company", "supplier"]);
                                        let contact = pick_first(s, &["contact_name", "contact", "person"]);
                                        let phone = pick_first(s, &["phone"]);
                                        let email = pick_first(s, &["email"]);
                                        let status = pick_first(s, &["status", "tier", "rating"]);
                                        let address = pick_first(s, &["address"]);
                                        let mut show_edit_modal = show_edit_modal.clone();
                                        let mut edit_id = edit_id.clone();
                                        let mut edit_name = edit_name.clone();
                                        let mut edit_contact = edit_contact.clone();
                                        let mut edit_phone = edit_phone.clone();
                                        let mut edit_email = edit_email.clone();
                                        let mut edit_address = edit_address.clone();
                                        let mut edit_status = edit_status.clone();
                                        let mut suppliers_state = suppliers_state.clone();
                                        let mut delete_msg = delete_msg.clone();
                                        let mut delete_error = delete_error.clone();
                                        let sid_for_delete = sid.clone();
                                        rsx!(
                                            tr {
                                                td { "{name}" }
                                                td { "{contact}" }
                                                td { "{phone}" }
                                                td { "{email}" }
                                                td { span { class: "data-chip", "{status}" } }
                                                td { div { class: "table-actions",
                                                    button {
                                                        class: "btn-small",
                                                        onclick: move |_| {
                                                            edit_id.set(Some(sid.clone()));
                                                            edit_name.set(name.clone());
                                                            edit_contact.set(contact.clone());
                                                            edit_phone.set(phone.clone());
                                                            edit_email.set(email.clone());
                                                            edit_address.set(address.clone());
                                                            edit_status.set(if status.is_empty() { "active".to_string() } else { status.clone() });
                                                            show_edit_modal.set(true);
                                                        },
                                                        "✏️ Edit"
                                                    }
                                                    button {
                                                        class: "btn-small btn-danger",
                                                        onclick: move |_| {
                                                            let mut updated = suppliers_state.read().clone();
                                                            updated.retain(|s| pick_first(s, &["id", "supplier_id"]) != sid_for_delete);
                                                            match save_extra_to_json("suppliers.json", &updated) {
                                                                Ok(_) => {
                                                                    suppliers_state.set(updated);
                                                                    delete_error.set(None);
                                                                    delete_msg.set(Some("✅ Supplier deleted.".to_string()));
                                                                    spawn(async move {
                                                                        sleep_ms(2_000).await;
                                                                        delete_msg.set(None);
                                                                    });
                                                                }
                                                                Err(err) => delete_error.set(Some(err)),
                                                            }
                                                        },
                                                        "🗑️ Delete"
                                                    }
                                                } }
                                            }
                                        )
                                    }) }
                                }
                            }
                        }
                    }
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Supplier" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }
                        div { class: "form-group",
                            label { "Supplier Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏭" }
                                input {
                                    value: "{edit_name}",
                                    oninput: move |e| edit_name.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Contact Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input {
                                    value: "{edit_contact}",
                                    oninput: move |e| edit_contact.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input {
                                    value: "{edit_phone}",
                                    oninput: move |e| edit_phone.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input {
                                    value: "{edit_email}",
                                    oninput: move |e| edit_email.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Address" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📍" }
                                input {
                                    value: "{edit_address}",
                                    oninput: move |e| edit_address.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select {
                                    value: "{edit_status}",
                                    onchange: move |e| edit_status.set(e.value()),
                                    option { value: "active", "Active" }
                                    option { value: "inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_supplier,
                                "Save Changes"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(false),
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ShiftsTab() -> Element {
    let store = get_store_fresh();
    let shifts = extra_list("shifts.json");
    let open_shifts = shifts.iter().filter(|s| value_is(s, "status", "open")).count();
    let closed_shifts = shifts.iter().filter(|s| value_is(s, "status", "closed")).count();

    rsx! {
        div { class: "content-card",
            h2 { "🕒 Shifts" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {shifts.len()}" }
                div { class: "ops-pill success", "Open: {open_shifts}" }
                div { class: "ops-pill warning", "Closed: {closed_shifts}" }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Shifts" }
                        span { class: "card-note", "{shifts.len()} shifts" }
                    }
                    div { class: "data-list",
                        if shifts.is_empty() {
                            div { class: "empty-state", "No shifts available." }
                        } else {
                            { shifts.iter().rev().take(6).map(|s| {
                                let staff = pick_first(s, &["staff_name", "staff_id", "user_id"]);
                                let time = pick_first(s, &["start_time", "start", "created_at"]);
                                let status = pick_first(s, &["status", "state"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{staff}" }
                                            span { "{time}" }
                                        }
                                        span { class: "data-chip", "{status}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Coverage Notes" }
                        span { class: "card-note", "Staffing" }
                    }
                    div { class: "empty-state", "Add coverage gaps, handoff notes, and approvals here." }
                }
            }
        }
    }
}

#[component]
fn SettingsTab() -> Element {
    let initial_settings = crate::config::ui_settings::get_ui_settings();
    let mut business_name = use_signal(|| initial_settings.business_name.clone());
    let mut currency = use_signal(|| initial_settings.currency.clone());
    let mut show_save_msg = use_signal(|| false);
    let mut save_error = use_signal(|| None::<String>);
    
    let handle_save = move |_| {
        let settings = crate::config::ui_settings::UiSettings {
            business_name: business_name.read().clone(),
            currency: currency.read().clone(),
        };
        match crate::config::ui_settings::save_ui_settings(&settings) {
            Ok(_) => {
                save_error.set(None);
                show_save_msg.set(true);
                spawn(async move {
                    sleep_ms(2_000).await;
                    show_save_msg.set(false);
                });
            }
            Err(err) => {
                save_error.set(Some(format!("Failed to save settings: {}", err)));
            }
        }
    };
    
    rsx! {
        div { class: "tab-content",
            h2 { "⚙️ Settings" }
            
            div { class: "settings-form",
                div { class: "form-group",
                    label { "Business Name" }
                    input {
                        placeholder: "Your store name",
                        value: "{business_name}",
                        oninput: move |e| business_name.set(e.value()),
                    }
                }
                
                div { class: "form-group",
                    label { "Currency" }
                    select {
                        value: "{currency}",
                        onchange: move |e| currency.set(e.value()),
                        option { value: "USD", "💵 USD ($)" }
                        option { value: "EUR", "💶 EUR (€)" }
                        option { value: "GBP", "💷 GBP (£)" }
                        option { value: "JPY", "💴 JPY (¥)" }
                    }
                }
                
                div { class: "form-group",
                    label { "Database Location" }
                    p { "📁 data/pos_data.db (Local SQLite)" }
                }
                
                div { class: "form-group",
                    label { "Image Storage" }
                    p { "📁 Products: data/images/products/" }
                    p { style: "margin-top: 5px;", "📁 Profiles: data/images/profiles/" }
                }
                
                if show_save_msg.read().clone() {
                    div { class: "message",
                        "✅ Settings saved successfully!"
                    }
                }
                if let Some(err) = save_error.read().clone() {
                    div { class: "message error",
                        "{err}"
                    }
                }
                
                button {
                    class: "btn btn-primary",
                    onclick: handle_save,
                    "💾 Save Settings"
                }
            }
        }
    }
}
