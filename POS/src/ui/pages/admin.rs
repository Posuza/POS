use dioxus::prelude::*;
use crate::data::models::user::User;
use crate::utils::formatters::{format_price, format_datetime};
use crate::ui::components::{Navbar, Sidebar, SalesTrend};
use crate::config::constants::APP_NAME;

// Sample product dataset shared between Dashboard and Products tab
const SAMPLE_PRODUCTS: &[(&str, &str, &str, f32, i32)] = &[
    ("123456789012", "Apple", "Fruits", 1.50, 100),
    ("123456789013", "Banana", "Fruits", 0.99, 150),
    ("123456789014", "Milk", "Dairy", 2.99, 50),
    ("123456789015", "Bread", "Bakery", 3.49, 75),
    ("123456789016", "Eggs", "Dairy", 2.50, 120),
    ("123456789017", "Orange", "Fruits", 1.29, 90),
    ("123456789018", "Yogurt", "Dairy", 1.99, 60),
    ("123456789019", "Croissant", "Bakery", 2.75, 40),
    ("123456789020", "Tomato", "Produce", 0.89, 200),
];

// Sample sales/transactions dataset for demo UI (expanded)
const SAMPLE_SALES: &[(&str, &str, f32, &str)] = &[
    ("tx001", "2026-01-20T09:12:00", 8.50, "staff"),
    ("tx002", "2026-01-20T10:05:00", 12.00, "staff"),
    ("tx003", "2026-01-21T11:15:22", 7.25, "staff"),
    ("tx004", "2026-01-21T12:30:10", 18.75, "staff"),
    ("tx005", "2026-01-22T09:45:50", 5.00, "staff"),
    ("tx006", "2026-01-22T14:20:05", 22.10, "admin"),
    ("tx007", "2026-01-23T10:00:00", 12.50, "staff"),
    ("tx008", "2026-01-24T16:40:00", 30.00, "admin"),
    ("tx009", "2026-01-25T09:05:30", 4.99, "staff"),
    ("tx010", "2026-01-26T13:12:45", 27.50, "staff"),
    ("tx011", "2026-01-27T18:22:10", 15.00, "admin"),
    ("tx012", "2026-01-28T08:55:00", 9.75, "staff"),
];

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

            div { class: "admin-page",
                div { class: "admin-container",
                    // Sidebar (use centralized Sidebar component)
                    div {
                        class: if *sidebar_open.read() { "admin-sidebar open" } else { "admin-sidebar" },
                        Sidebar {
                            active: match *active_tab.read() {
                                AdminTab::Dashboard => "dashboard".to_string(),
                                AdminTab::Sales => "sales".to_string(),
                                AdminTab::Products => "products".to_string(),
                                AdminTab::Staff => "staff".to_string(),
                                AdminTab::Settings => "settings".to_string(),
                            },
                            on_select: { move |s: String| {
                                match s.as_str() {
                                    "dashboard" => active_tab.set(AdminTab::Dashboard),
                                    "sales" => active_tab.set(AdminTab::Sales),
                                    "products" => active_tab.set(AdminTab::Products),
                                    "staff" => active_tab.set(AdminTab::Staff),
                                    "settings" => active_tab.set(AdminTab::Settings),
                                    _ => (),
                                }
                                sidebar_open.set(false);
                            } },
                            header: "Admin Panel".to_string(),
                        }
                    }

                    // Content
                    div { class: "admin-main",
                        match *active_tab.read() {
                            AdminTab::Dashboard => rsx! { DashboardTab {} },
                            AdminTab::Sales => rsx! { SalesTab {} },
                            AdminTab::Products => rsx! { ProductsTab {} },
                            AdminTab::Staff => rsx! { StaffTab { user: user.clone() } },
                            AdminTab::Settings => rsx! { SettingsTab {} },
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
    Sales,
    Products,
    Staff,
    Settings,
}

#[component]
fn SidebarMenu(_active_tab: Signal<AdminTab>, _on_select: EventHandler<AdminTab>) -> Element {
    // Deprecated: sidebar moved to `components::Sidebar`.
    rsx! { div {} }
}

// `MenuButton` removed — sidebar menu entries are now rendered by `components::Sidebar`.

#[component]
fn DashboardTab() -> Element {
    // compute summary stats from SAMPLE_PRODUCTS
    let total_products = SAMPLE_PRODUCTS.len();
    let total_stock: i32 = SAMPLE_PRODUCTS.iter().map(|p| p.4).sum();
    let total_sales_value: f32 = SAMPLE_PRODUCTS.iter().map(|p| p.3 * p.4 as f32).sum();
    let active_users = 5; // placeholder; replace with real user count when available

    let total_products_s = format!("{}", total_products);
    let total_stock_s = format!("{}", total_stock);
    let total_sales_s = format!("${:.2}", total_sales_value);
    let active_users_s = format!("{}", active_users);

    // compute unique categories for a dashboard stat card
    let mut cats: Vec<&str> = Vec::new();
    for p in SAMPLE_PRODUCTS.iter() {
        if !cats.contains(&p.2) {
            cats.push(p.2);
        }
    }
    let categories_s = format!("{}", cats.len());

    let _items_line = format!("Items in products table: {}", total_products_s);
    let _stock_line = format!("Total units in stock: {}", total_stock_s);
    let _sales_line = format!("Estimated total sales value: {}", total_sales_s);

    // series for chart: use SAMPLE_SALES totals (simple demo series)
    let sales_series: Vec<f32> = SAMPLE_SALES.iter().map(|s| s.2).collect();
    // (removed second demo "Sales (Adj)" series)

    // create a demo "stock" series by cycling product stock values to match sales length
    let stock_series: Vec<f32> = (0..SAMPLE_SALES.len()).map(|i| SAMPLE_PRODUCTS[i % SAMPLE_PRODUCTS.len()].4 as f32).collect();

    // create a demo "users" series (mock concurrent users per tick)
    let users_series: Vec<f32> = vec![3.0, 4.0, 2.0, 5.0, 3.0, 8.0, 6.0, 10.0, 2.0, 4.0, 7.0, 5.0, 6.0];

    // x-axis labels: format timestamps for the chart (uses format_datetime)
    let x_labels: Vec<String> = SAMPLE_SALES.iter().map(|s| format_datetime(s.1)).collect();

    rsx! {
        div { class: "admin-main",
            div { class: "content-card",
                div { class: "dashboard-header",
                    h2 { "📊 Dashboard" }
                }

                div { class: "stat-cards",
                    StatCard { icon: "🔍", label: "Total Stock", value: total_stock_s.clone() }
                    StatCard { icon: "💰", label: "Total Sales Value", value: total_sales_s.clone() }
                    StatCard { icon: "📦", label: "Products", value: total_products_s.clone() }
                    StatCard { icon: "🗂️", label: "Categories", value: categories_s.clone() }
                    StatCard { icon: "👥", label: "Active Users", value: active_users_s.clone() }
                }

                SalesTrend { series: vec![
                    ("Sales".to_string(), sales_series.clone()),
                    ("Stock".to_string(), stock_series.clone()),
                    ("Users".to_string(), users_series.clone()),
                ], x_labels: Some(x_labels.clone()) }
            }
        }
    }
}

// `SalesTrend` component moved to `src/ui/components/sales_trend.rs`

#[component]
fn StatCard(icon: &'static str, label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "stat-card",
            div { class: "icon", "{icon}" }
            div { class: "stat-card-body",
                h4 { class: "stat-label", "{label}" }
                p { class: "stat-value", "{value}" }
            }
        }
    }
}

#[component]
fn ProductsTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut product_name = use_signal(|| String::new());
    let mut product_barcode = use_signal(|| String::new());
    let mut product_category = use_signal(|| String::new());
    let mut product_price = use_signal(|| String::new());
    let mut product_stock = use_signal(|| String::new());
    let mut product_image = use_signal(|| None::<String>);
    
    let handle_add_product = move |_| {
        if product_name.read().is_empty() || product_barcode.read().is_empty() {
            return;
        }
        // In real app, save to database
        product_name.set(String::new());
        product_barcode.set(String::new());
        product_category.set(String::new());
        product_price.set(String::new());
        product_stock.set(String::new());
        product_image.set(None);
        show_add_modal.set(false);
    };
    
    // Render rows from SAMPLE_PRODUCTS inline (rsx expects an iterator)

    rsx! {
        div { class: "content-card",
            div { class: "products-header",
                h2 { "📦 Manage Products" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_add_modal.set(true),
                    "+ Add Product"
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
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { SAMPLE_PRODUCTS.iter().map(|p| rsx!{
                            tr {
                                td { "{p.0}" }
                                td { "{p.1}" }
                                td { "{p.2}" }
                                td { "{format_price(p.3)}" }
                                td { "{p.4}" }
                                td { div { class: "table-actions",
                                    button { class: "btn-small", "✏️ Edit" }
                                    button { class: "btn-small btn-danger", "🗑️ Delete" }
                                } }
                            }
                        }) }
                    }
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
                            input {
                                placeholder: "e.g., Apple",
                                value: "{product_name}",
                                oninput: move |e| product_name.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Barcode *" }
                            input {
                                placeholder: "e.g., 123456789012",
                                value: "{product_barcode}",
                                oninput: move |e| product_barcode.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Category" }
                            input {
                                placeholder: "e.g., Fruits",
                                value: "{product_category}",
                                oninput: move |e| product_category.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Price ($)" }
                            input {
                                placeholder: "e.g., 1.50",
                                value: "{product_price}",
                                oninput: move |e| product_price.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Stock" }
                            input {
                                placeholder: "e.g., 100",
                                value: "{product_stock}",
                                oninput: move |e| product_stock.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Product Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                input {
                                    r#type: "file",
                                    accept: "image/*",
                                    onchange: move |_e| {
                                        // In real app, handle file upload to data/images/products/
                                    },
                                }
                            }
                            if let Some(img) = product_image.read().clone() {
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
        }
    }
}

#[component]
fn SalesTab() -> Element {
    rsx! {
        div { class: "content-card",
            h2 { "💸 Sales / Transactions" }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Tx ID" }
                            th { "Timestamp" }
                            th { "Total" }
                            th { "Cashier" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { SAMPLE_SALES.iter().map(|s| rsx!{
                            tr {
                                td { "{s.0}" }
                                td { "{format_datetime(s.1)}" }
                                td { "{format_price(s.2)}" }
                                td { "{s.3}" }
                                td { div { class: "table-actions",
                                    button { class: "btn-small", "🔍 View" }
                                    button { class: "btn-small btn-danger", "🗑️ Void" }
                                } }
                            }
                        }) }
                    }
                }
            }
        }
    }
}

#[component]
fn StaffTab(user: User) -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut staff_username = use_signal(|| String::new());
    let mut staff_email = use_signal(|| String::new());
    let mut staff_password = use_signal(|| String::new());
    let mut staff_image = use_signal(|| None::<String>);
    
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
                        tr {
                            td { "👤" }
                            td { "staff" }
                            td { "staff@pos.local" }
                            td { "👤 Staff" }
                            td { "✅ Active" }
                            td {
                                div { class: "table-actions",
                                    button { class: "btn-small", "✏️ Edit" }
                                    button { class: "btn-small btn-danger", "🗑️ Remove" }
                                }
                            }
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
fn SettingsTab() -> Element {
    let mut business_name = use_signal(|| "My Store".to_string());
    let mut currency = use_signal(|| "USD".to_string());
    let mut show_save_msg = use_signal(|| false);
    
    let handle_save = move |_| {
        show_save_msg.set(true);
        spawn(async move {
            gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
            show_save_msg.set(false);
        });
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
                
                button {
                    class: "btn btn-primary",
                    onclick: handle_save,
                    "💾 Save Settings"
                }
            }
        }
    }
}
