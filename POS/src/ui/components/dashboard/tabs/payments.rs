use super::prelude::*;

#[component]
pub(crate) fn PaymentsTab() -> Element {
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
