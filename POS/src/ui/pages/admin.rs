use dioxus::prelude::*;
use crate::data::models::user::User;
use crate::data::json_store::get_store;
use crate::utils::formatters::{format_price, format_datetime};
use crate::ui::components::{Navbar, SalesTrend};
use crate::config::constants::APP_NAME;
use std::time::Duration;
use std::collections::HashMap;


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
                    let tab_label = match *active_tab.read() {
                        AdminTab::Dashboard => "Dashboard",
                        AdminTab::Sales => "Sales",
                        AdminTab::Products => "Products",
                        AdminTab::Staff => "Staff",
                        AdminTab::Customers => "Customers",
                        AdminTab::Inventory => "Inventory",
                        AdminTab::Payments => "Payments",
                        AdminTab::Access => "Access Control",
                        AdminTab::Suppliers => "Suppliers",
                        AdminTab::Shifts => "Shifts",
                        AdminTab::Settings => "Settings",
                    };
                    let tab_subtitle = match *active_tab.read() {
                        AdminTab::Dashboard => "Live operational overview and insights.",
                        AdminTab::Sales => "Review transactions and revenue performance.",
                        AdminTab::Products => "Manage inventory and product catalog.",
                        AdminTab::Staff => "Control staff access and schedules.",
                        AdminTab::Customers => "Track customer profiles and engagement.",
                        AdminTab::Inventory => "Monitor stock movements and scan activity.",
                        AdminTab::Payments => "Review payment records and settlement status.",
                        AdminTab::Access => "Manage roles, permissions, and assignments.",
                        AdminTab::Suppliers => "Coordinate supplier contacts and performance.",
                        AdminTab::Shifts => "Coordinate staffing coverage and schedules.",
                        AdminTab::Settings => "Configure store preferences and system settings.",
                    };
                    let store = get_store();
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
                            AdminTab::Sales => rsx! { SalesTab {} },
                            AdminTab::Products => rsx! { ProductsTab {} },
                            AdminTab::Staff => rsx! { StaffTab {} },
                            AdminTab::Customers => rsx! { CustomersTab {} },
                            AdminTab::Inventory => rsx! { InventoryTab {} },
                            AdminTab::Payments => rsx! { PaymentsTab {} },
                            AdminTab::Access => rsx! { AccessTab {} },
                            AdminTab::Suppliers => rsx! { SuppliersTab {} },
                            AdminTab::Shifts => rsx! { ShiftsTab {} },
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
    Customers,
    Inventory,
    Payments,
    Access,
    Suppliers,
    Shifts,
    Settings,
}

#[component]
fn SidebarMenu(active_tab: Signal<AdminTab>, on_select: EventHandler<AdminTab>, is_open: Signal<bool>, on_close: EventHandler<()>) -> Element {
    let drawer_open = *is_open.read();
    let current_tab = *active_tab.read();
    let menu_items = [
        (AdminTab::Dashboard, "dashboard", "Dashboard", "📊"),
        (AdminTab::Sales, "sales", "Sales", "💸"),
        (AdminTab::Products, "products", "Products", "📦"),
        (AdminTab::Staff, "staff", "Staff", "👥"),
        (AdminTab::Customers, "customers", "Customers", "🧑‍🤝‍🧑"),
        (AdminTab::Inventory, "inventory", "Inventory", "🗄️"),
        (AdminTab::Payments, "payments", "Payments", "💳"),
        (AdminTab::Access, "access", "Access", "🛡️"),
        (AdminTab::Suppliers, "suppliers", "Suppliers", "🚚"),
        (AdminTab::Shifts, "shifts", "Shifts", "🕒"),
        (AdminTab::Settings, "settings", "Settings", "⚙️"),
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
                h3 { "Admin Menu" }
                button {
                    class: "admin-sidebar-close",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            nav {
                class: "admin-sidebar-nav",
                for (tab, _id, label, icon) in menu_items {
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

// `SidebarMenu` renders the floating admin drawer entries directly.

fn extra_list<'a>(store: &'a crate::data::json_store::JsonStore, key: &str) -> Vec<&'a serde_json::Value> {
    match store.extras.get(key) {
        Some(serde_json::Value::Array(items)) => items.iter().collect(),
        _ => Vec::new(),
    }
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
    let store = get_store();
    let products = &store.products;
    let sales = &store.sales;
    let users = &store.users;
    let sale_items = extra_list(store, "sale_items.json");

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
    let total_sales_s = format!("${:.2}", total_sales_value);
    let total_revenue_s = format!("${:.2}", total_revenue);
    let active_users_s = format!("{}", active_users);
    let inactive_users_s = format!("{}", inactive_users);
    let total_transactions_s = format!("{} transactions", total_transactions);
    let average_ticket_s = format!("{} avg ticket", format_price(average_ticket));

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
    let low_stock_count = products.iter().filter(|p| p.quantity < 60).count();
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();

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

    rsx! {
        section { class: "dashboard-stack dashboard-overview",
            div { class: "overview-header",
                div { class: "overview-title",
                    p { class: "overview-kicker", "Overview" }
                    h2 { "Detailed information about your store" }
                    p { class: "overview-subtitle", "{greeting} · {total_transactions_s} · {average_ticket_s}" }
                }
                div { class: "overview-actions",
                    button { class: "icon-btn", aria_label: "Search dashboard", "🔍" }
                    button { class: "icon-btn", aria_label: "Notifications", "🔔" }
                    div { class: "overview-profile",
                        if let Some(img) = user.profile_image.clone() {
                            img { class: "profile-avatar", src: "{img}" }
                        } else {
                            div { class: "profile-avatar profile-avatar-fallback", "{profile_initials}" }
                        }
                        div { class: "overview-profile-meta",
                            span { class: "overview-name", "{user.username}" }
                            span { class: "overview-role", "{role_label}" }
                        }
                    }
                }
            }

            div { class: "overview-grid",
                div { class: "overview-main",
                    div { class: "content-card analytics-card",
                        div { class: "analytics-header",
                            div {
                                h3 { "Sales Analytics" }
                                p { "Revenue, stock, and traffic signals" }
                            }
                            div { class: "analytics-controls",
                                button { class: "pill-btn active", "This year" }
                                button { class: "pill-btn", "Last 30 days" }
                                button { class: "pill-btn", "Custom" }
                            }
                        }

                        div { class: "analytics-legend",
                            span { class: "legend-dot sales", "Sales" }
                            span { class: "legend-dot items", "Items Sold" }
                            span { class: "legend-dot ticket", "Avg Ticket" }
                        }

                        SalesTrend { series: vec![
                            ("Sales".to_string(), sales_series.clone()),
                            ("Items Sold".to_string(), items_series.clone()),
                            ("Avg Ticket".to_string(), ticket_series.clone()),
                        ], x_labels: Some(x_labels.clone()) }

                        div { class: "analytics-summary",
                            div { class: "summary-block",
                                span { "Total revenue" }
                                strong { "{total_revenue_s}" }
                            }
                            div { class: "summary-block",
                                span { "Avg ticket" }
                                strong { "{average_ticket_s}" }
                            }
                            div { class: "summary-block",
                                span { "Last updated" }
                                strong { "{last_updated}" }
                            }
                        }
                    }

                    div { class: "content-card country-card",
                        div { class: "card-header-row",
                            h3 { "Top Categories" }
                            span { class: "card-note", "{categories_s} total" }
                        }
                        div { class: "category-grid",
                            div { class: "donut-wrap",
                                div { class: "donut-chart", style: "{donut_style}" }
                                div { class: "donut-center",
                                    strong { "{categories_s}" }
                                    span { "Categories" }
                                }
                            }
                            div { class: "category-list",
                                if category_vec.is_empty() {
                                    div { class: "empty-state",
                                        span { "No categories yet." }
                                    }
                                } else {
                                    { category_vec.iter().take(4).map(|(name, total)| {
                                        let pct = ((*total / max_category) * 100.0).round() as i32;
                                        let total_s = format_price(*total);
                                        rsx!(
                                            div { class: "category-item",
                                                div { class: "category-main",
                                                    strong { "{name}" }
                                                    span { "{total_s}" }
                                                }
                                                div { class: "category-bar",
                                                    div { class: "category-fill", style: "width: {pct}%;" }
                                                }
                                            }
                                        )
                                    }) }
                                }
                            }
                        }
                    }

                    div { class: "content-card activity-card",
                        div { class: "card-header-row",
                            h3 { "Last Activity" }
                            a { class: "card-link", href: "#activity", "View all" }
                        }
                        div { class: "activity-list",
                            if sales.is_empty() {
                                div { class: "empty-state",
                                    span { "No recent activity yet." }
                                }
                            } else {
                                { sales.iter().rev().take(4).map(|s| {
                                    let amount = format_price(s.total);
                                    let when = format_datetime(&s.created_at);
                                    let cashier = s.cashier_id.clone();
                                    let initials = initials_for(&cashier);
                                    rsx!(
                                        div { class: "activity-item",
                                            div { class: "activity-avatar", "{initials}" }
                                            div { class: "activity-body",
                                                p { class: "activity-title", "Sale {s.id} · {cashier}" }
                                                span { class: "activity-meta", "{when}" }
                                            }
                                            div { class: "activity-value", "{amount}" }
                                        }
                                    )
                                }) }
                            }
                        }
                    }

                    div { class: "content-card product-card",
                        div { class: "card-header-row",
                            h3 { "Product Performance" }
                            span { class: "card-note", "Top selling items" }
                        }
                        div { class: "table-container table-compact",
                            table { class: "product-table",
                                thead {
                                    tr {
                                        th { "Item" }
                                        th { "Sold" }
                                        th { "Stock" }
                                        th { "Avg Price" }
                                        th { "Revenue" }
                                        th { "Status" }
                                    }
                                }
                                tbody {
                                    { product_sales_vec.iter().take(6).map(|(id, name, sold_qty, revenue)| {
                                        let product = product_lookup.get(id);
                                        let stock = product.map(|p| p.quantity).unwrap_or(0);
                                        let avg_price = if *sold_qty == 0 { 0.0 } else { revenue / *sold_qty as f32 };
                                        let barcode = product.map(|p| p.barcode.clone()).unwrap_or_else(|| "—".to_string());
                                        let status = if stock == 0 { "Out" } else if stock < 40 { "Low" } else { "Good" };
                                        let status_class = if stock == 0 { "status-chip danger" } else if stock < 40 { "status-chip warning" } else { "status-chip ok" };
                                        let initials = initials_for(name);
                                        let revenue_s = format_price(*revenue);
                                        rsx!(
                                            tr {
                                                td {
                                                    div { class: "item-cell",
                                                        if let Some(img) = product.and_then(|p| p.product_image.clone()) {
                                                            img { class: "item-avatar", src: "{img}" }
                                                        } else {
                                                            span { class: "item-avatar fallback", "{initials}" }
                                                        }
                                                        div { class: "item-meta",
                                                            strong { "{name}" }
                                                            span { "{barcode}" }
                                                        }
                                                    }
                                                }
                                                td { "{sold_qty}" }
                                                td { "{stock}" }
                                                td { "{format_price(avg_price)}" }
                                                td { "{revenue_s}" }
                                                td { span { class: "{status_class}", "{status}" } }
                                            }
                                        )
                                    }) }
                                }
                            }
                        }
                    }
                }

                div { class: "overview-side",
                    div { class: "content-card metric-card metric-dark",
                        div { class: "metric-header",
                            h4 { "Total Customers" }
                            span { class: "metric-tag", "↑ 7.9%" }
                        }
                        p { class: "metric-value", "{users.len()}" }
                        div { class: "metric-sub", "{active_users_s} active · {inactive_users_s} inactive" }
                        div { class: "split-bar",
                            span { class: "split active", style: "width: {active_pct}%;" }
                            span { class: "split inactive", style: "width: {inactive_pct}%;" }
                        }
                        div { class: "split-legend",
                            span { class: "legend-dot active", "Active" }
                            span { class: "legend-dot inactive", "Inactive" }
                        }
                    }

                    div { class: "content-card metric-card revenue-card",
                        div { class: "metric-header",
                            h4 { "Total Revenue" }
                            span { class: "metric-tag success", "↑ 10.2%" }
                        }
                        p { class: "metric-value", "{total_revenue_s}" }
                        div { class: "metric-sub", "Compared to last month" }
                        div { class: "spark-bars",
                            if spark_heights.is_empty() {
                                { (0..8).map(|_| rsx!( span { class: "spark-bar" } )) }
                            } else {
                                { spark_heights.iter().map(|h| rsx!(
                                    span { class: "spark-bar", style: "height: {h}%;" }
                                )) }
                            }
                        }
                    }

                    div { class: "content-card metric-card orders-card",
                        div { class: "metric-header",
                            h4 { "Total Orders" }
                            span { class: "metric-tag warning", "↑ 5.1%" }
                        }
                        p { class: "metric-value", "{total_transactions}" }
                        div { class: "metric-sub", "{total_products_s} products listed" }
                        div { class: "orders-wave" }
                    }

                    div { class: "content-card alerts-card",
                        div { class: "card-header-row",
                            h3 { "Ops Alerts" }
                            span { class: "card-note", "{alerts.len()} items" }
                        }
                        div { class: "alert-list",
                            { alerts.iter().map(|(tone, title, meta)| {
                                rsx!(
                                    div { class: "alert-item {tone}",
                                        span { class: "alert-title", "{title}" }
                                        span { class: "alert-meta", "{meta}" }
                                    }
                                )
                            }) }
                        }
                    }

                    button { class: "export-btn", "Export statistics" }
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
fn ProductsTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut product_name = use_signal(|| String::new());
    let mut product_barcode = use_signal(|| String::new());
    let mut product_category = use_signal(|| String::new());
    let mut product_price = use_signal(|| String::new());
    let mut product_stock = use_signal(|| String::new());
    let mut product_image = use_signal(|| None::<String>);
    let store = get_store();
    let products = &store.products;
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
    let total_revenue_s = format!("${:.2}", total_revenue);
    let average_ticket_s = format!("{} avg ticket", format_price(average_ticket));
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());
    
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
    
    // Render rows from JSON products inline (rsx expects an iterator)
    let mut products_sorted: Vec<_> = products.iter().cloned().collect();
    products_sorted.sort_by(|a, b| {
        let a_val = a.price * a.quantity as f32;
        let b_val = b.price * b.quantity as f32;
        b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
    });
    let low_stock_items: Vec<_> = products
        .iter()
        .filter(|p| p.quantity < 60)
        .cloned()
        .collect();
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock_count = low_stock_items.len();

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
                        { products.iter().map(|p| rsx!{
                            tr {
                                td { "{p.barcode}" }
                                td { "{p.name}" }
                                td { "{p.category}" }
                                td { "{format_price(p.price)}" }
                                td { "{p.quantity}" }
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
fn SalesTab() -> Element {
    let store = get_store();
    let sales = &store.sales;
    let sale_items = extra_list(store, "sale_items.json");
    let scans = &store.scans;
    let completed_sales = sales.iter().filter(|s| s.status == "completed").count();
    let pending_sales = sales.iter().filter(|s| s.status != "completed").count();

    rsx! {
        div { class: "content-card",
            h2 { "💸 Sales / Transactions" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {sales.len()}" }
                div { class: "ops-pill success", "Completed: {completed_sales}" }
                div { class: "ops-pill warning", "Pending: {pending_sales}" }
            }

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
                        if sales.is_empty() {
                            tr {
                                td { colspan: "5",
                                    div { class: "empty-state", "No sales yet." }
                                }
                            }
                        } else {
                            { sales.iter().map(|s| rsx!{
                                tr {
                                    td { "{s.id}" }
                                    td { "{format_datetime(&s.created_at)}" }
                                    td { "{format_price(s.total)}" }
                                    td { "{s.cashier_id}" }
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
        }
    }
}

#[component]
fn StaffTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut staff_username = use_signal(|| String::new());
    let mut staff_email = use_signal(|| String::new());
    let mut staff_password = use_signal(|| String::new());
    let mut staff_image = use_signal(|| None::<String>);
    let store = get_store();
    let users = &store.users;
    
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
                        if users.is_empty() {
                            tr {
                                td { colspan: "6",
                                    div { class: "empty-state", "No staff users found." }
                                }
                            }
                        } else {
                            { users.iter().map(|u| {
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
                                rsx!(
                                    tr {
                                        td {
                                            if let Some(img) = u.profile_image.clone() {
                                                img { class: "table-avatar", src: "{img}" }
                                            } else {
                                                span { class: "table-avatar-fallback", "{initials}" }
                                            }
                                        }
                                        td { "{u.username}" }
                                        td { "{u.email}" }
                                        td { "{role_label}" }
                                        td { "{status_label}" }
                                        td {
                                            div { class: "table-actions",
                                                button { class: "btn-small", "✏️ Edit" }
                                                button { class: "btn-small btn-danger", "🗑️ Remove" }
                                            }
                                        }
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
    let store = get_store();
    let customers = extra_list(store, "customers.json");
    let active_customers = customers.iter().filter(|c| value_is(c, "status", "active")).count();

    rsx! {
        div { class: "content-card",
            h2 { "🧑‍🤝‍🧑 Customers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {customers.len()}" }
                div { class: "ops-pill success", "Active: {active_customers}" }
            }

            div { class: "data-ops-grid",
                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Recent Customers" }
                        span { class: "card-note", "{customers.len()} records" }
                    }
                    div { class: "data-list",
                        if customers.is_empty() {
                            div { class: "empty-state", "No customer data found." }
                        } else {
                            { customers.iter().rev().take(6).map(|c| {
                                let name = pick_first(c, &["name", "full_name", "username", "id"]);
                                let email = pick_first(c, &["email", "contact_email", "phone"]);
                                let tier = pick_first(c, &["tier", "segment", "status"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{email}" }
                                        }
                                        span { class: "data-chip", "{tier}" }
                                    }
                                )
                            }) }
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
                            strong { "{customers.len().saturating_sub(active_customers)}" }
                        }
                        div { class: "data-stat",
                            span { "Records" }
                            strong { "{customers.len()}" }
                        }
                    }
                    div { class: "empty-state", "Use this space for loyalty analytics and segments." }
                }
            }
        }
    }
}

#[component]
fn InventoryTab() -> Element {
    let store = get_store();
    let products = &store.products;
    let movements = extra_list(store, "inventory_movements.json");
    let categories = extra_list(store, "categories.json");
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock = products.iter().filter(|p| p.quantity < 60).count();

    rsx! {
        div { class: "content-card",
            h2 { "🗄️ Inventory" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Products: {products.len()}" }
                div { class: "ops-pill warning", "Low stock: {low_stock}" }
                div { class: "ops-pill danger", "Out: {out_of_stock}" }
                div { class: "ops-pill", "Movements: {movements.len()}" }
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
        }
    }
}

#[component]
fn PaymentsTab() -> Element {
    let store = get_store();
    let payments = extra_list(store, "payments.json");
    let paid = payments.iter().filter(|p| value_is(p, "status", "paid")).count();
    let pending = payments.iter().filter(|p| value_is(p, "status", "pending")).count();
    let failed = payments.iter().filter(|p| value_is(p, "status", "failed")).count();

    rsx! {
        div { class: "content-card",
            h2 { "💳 Payments" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {payments.len()}" }
                div { class: "ops-pill success", "Paid: {paid}" }
                div { class: "ops-pill warning", "Pending: {pending}" }
                div { class: "ops-pill danger", "Failed: {failed}" }
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
                                let amount = pick_first(p, &["amount", "total", "value"]);
                                let status = pick_first(p, &["status", "state"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{method}" }
                                            span { "{status}" }
                                        }
                                        span { class: "data-chip", "{amount}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Settlement Notes" }
                        span { class: "card-note", "Ops checklist" }
                    }
                    div { class: "empty-state", "Add settlement runs, reconciliation notes, and payout tracking here." }
                }
            }
        }
    }
}

#[component]
fn AccessTab() -> Element {
    let store = get_store();
    let roles = extra_list(store, "roles.json");
    let permissions = extra_list(store, "permissions.json");
    let role_permissions = extra_list(store, "role_permissions.json");
    let user_roles = extra_list(store, "user_roles.json");

    rsx! {
        div { class: "content-card",
            h2 { "🛡️ Access Control" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Roles: {roles.len()}" }
                div { class: "ops-pill", "Permissions: {permissions.len()}" }
                div { class: "ops-pill", "Role links: {role_permissions.len()}" }
                div { class: "ops-pill", "User links: {user_roles.len()}" }
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
        }
    }
}

#[component]
fn SuppliersTab() -> Element {
    let store = get_store();
    let suppliers = extra_list(store, "suppliers.json");
    let active_suppliers = suppliers.iter().filter(|s| value_is(s, "status", "active")).count();

    rsx! {
        div { class: "content-card",
            h2 { "🚚 Suppliers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {suppliers.len()}" }
                div { class: "ops-pill success", "Active: {active_suppliers}" }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Supplier Directory" }
                        span { class: "card-note", "{suppliers.len()} records" }
                    }
                    div { class: "data-list",
                        if suppliers.is_empty() {
                            div { class: "empty-state", "No suppliers available." }
                        } else {
                            { suppliers.iter().take(8).map(|s| {
                                let name = pick_first(s, &["name", "company", "supplier"]);
                                let contact = pick_first(s, &["email", "phone", "contact"]);
                                let status = pick_first(s, &["status", "tier", "rating"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{contact}" }
                                        }
                                        span { class: "data-chip", "{status}" }
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
fn ShiftsTab() -> Element {
    let store = get_store();
    let shifts = extra_list(store, "shifts.json");
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
    let mut business_name = use_signal(|| "My Store".to_string());
    let mut currency = use_signal(|| "USD".to_string());
    let mut show_save_msg = use_signal(|| false);
    
    let handle_save = move |_| {
        show_save_msg.set(true);
        spawn(async move {
            sleep_ms(2_000).await;
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
