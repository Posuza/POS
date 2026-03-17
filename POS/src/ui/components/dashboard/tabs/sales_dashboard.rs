use super::prelude::*;

#[component]
pub(crate) fn SalesDashboardTab() -> Element {
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
    let voided_sales = sales
        .iter()
        .filter(|s| s.status.eq_ignore_ascii_case("voided"))
        .count();
    let paid_sales = sales
        .iter()
        .filter(|s| {
            s.status.eq_ignore_ascii_case("paid") || s.status.eq_ignore_ascii_case("completed")
        })
        .count();
    let pending_sales = total_transactions.saturating_sub(paid_sales + voided_sales);
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());

    let mut sales_by_day: HashMap<String, (f32, i32)> = HashMap::new();
    for s in sales.iter() {
        let day = s
            .created_at
            .split('T')
            .next()
            .unwrap_or(&s.created_at)
            .to_string();
        let entry = sales_by_day.entry(day).or_insert((0.0, 0));
        entry.0 += s.total;
        entry.1 += 1;
    }
    let mut items_by_day: HashMap<String, i32> = HashMap::new();
    let sales_by_id: HashMap<String, &crate::data::json_store::SaleRecord> =
        sales.iter().map(|s| (s.id.clone(), s)).collect();
    for item in sale_items.iter() {
        let sale_id = pick_first(item, &["sale_id", "saleId", "sale"]);
        let day = sales_by_id
            .get(&sale_id)
            .map(|s| {
                s.created_at
                    .split('T')
                    .next()
                    .unwrap_or(&s.created_at)
                    .to_string()
            })
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
    let product_lookup: HashMap<String, &Product> =
        products.iter().map(|p| (p.id.clone(), p)).collect();
    for item in sale_items.iter() {
        let product_id = pick_first(item, &["product_id", "productId", "id"]);
        let name = product_lookup
            .get(&product_id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| {
                pick_first(
                    item,
                    &["name", "product_name", "product_id", "productId", "id"],
                )
            });
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
        let amount = value_f32(p, "amount")
            .or_else(|| value_f32(p, "total"))
            .unwrap_or(0.0);
        let date = pick_first(p, &["paid_at", "created_at", "date"]);
        payment_rows.push((method, format_price(amount), date));
    }

    rsx! {
        DashboardContainer {
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
