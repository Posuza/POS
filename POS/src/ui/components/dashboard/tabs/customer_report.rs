use super::prelude::*;

#[component]
pub(crate) fn CustomerReportTab() -> Element {
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
