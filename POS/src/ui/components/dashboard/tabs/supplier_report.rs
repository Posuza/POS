use super::prelude::*;

#[component]
pub(crate) fn SupplierReportTab() -> Element {
    let _store = get_store_fresh();
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
