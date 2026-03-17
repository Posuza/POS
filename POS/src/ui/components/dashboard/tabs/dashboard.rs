use super::prelude::*;

#[component]
pub(crate) fn DashboardTab(user: User) -> Element {
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
        DashboardContainer {
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
