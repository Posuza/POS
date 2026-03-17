use super::prelude::*;

#[component]
pub(crate) fn StockTransferTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let rows: Vec<(String, String, String, String)> = movements
        .iter()
        .filter(|item| value_is(item, "movement_type", "out"))
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let product = product_lookup
                .get(&product_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let reason = pick_first(item, &["reason"]);
            let date = pick_first(item, &["created_at"]);
            (product, qty, reason, date)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search transfers..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Add Transfer" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Qty" }
                            th { "Reason" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, qty, reason, date) in rows {
                            tr {
                                td { "{product}" }
                                td { "{qty}" }
                                td { "{reason}" }
                                td { "{date}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}
