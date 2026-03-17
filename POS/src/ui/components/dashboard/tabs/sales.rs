use super::prelude::*;

#[component]
pub(crate) fn SalesTab() -> Element {
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
