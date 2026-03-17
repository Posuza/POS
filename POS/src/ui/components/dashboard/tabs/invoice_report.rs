use super::prelude::*;

#[component]
pub(crate) fn InvoiceReportTab() -> Element {
    let store = get_store_fresh();
    let rows: Vec<(String, String, String, String, String)> = store
        .sales
        .iter()
        .map(|s| {
            let customer = s
                .customer_id
                .clone()
                .unwrap_or_else(|| "Walk-in".to_string());
            let amount = format_price(s.total);
            let paid = if s.status.to_lowercase() == "paid" {
                amount.clone()
            } else {
                "$0.00".to_string()
            };
            let due = if s.status.to_lowercase() == "paid" {
                "$0.00".to_string()
            } else {
                amount.clone()
            };
            (
                s.receipt_no.clone(),
                customer,
                amount,
                paid,
                s.status.clone(),
            )
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
