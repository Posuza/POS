use super::prelude::*;

#[component]
pub(crate) fn SalesReportTab() -> Element {
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
            let customer = s
                .customer_id
                .clone()
                .unwrap_or_else(|| "Walk-in".to_string());
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
                        input { r#type: "date", value: "{start_date.read()}", oninput: move |e| start_date.set(e.value().clone()) }
                    }
                }
                div { class: "form-field",
                    label { "End Date" }
                    div { class: "input-group input-right",
                        span { class: "input-icon-right", "📅" }
                        input { r#type: "date", value: "{end_date.read()}", oninput: move |e| end_date.set(e.value().clone()) }
                    }
                }
                div { class: "form-field",
                    label { "Status" }
                    select { value: "{status_filter.read()}", onchange: move |e| status_filter.set(e.value().clone()),
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
