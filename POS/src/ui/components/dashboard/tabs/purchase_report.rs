use super::prelude::*;

#[component]
pub(crate) fn PurchaseReportTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let rows: Vec<(String, String, String, String)> = movements
        .iter()
        .filter(|item| value_is(item, "movement_type", "in"))
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let product = product_lookup.get(&product_id).cloned().unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let date = pick_first(item, &["created_at"]);
            let amount = pick_first(item, &["amount", "total"]);
            (product, qty, date, amount)
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search purchases..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Purchase Qty" }
                            th { "Purchase Date" }
                            th { "Amount" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, qty, date, amount) in rows {
                            tr {
                                td { "{product}" }
                                td { "{qty}" }
                                td { "{date}" }
                                td { "{amount}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}
