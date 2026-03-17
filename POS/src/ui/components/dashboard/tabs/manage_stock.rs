use super::prelude::*;

#[component]
pub(crate) fn ManageStockTab() -> Element {
    let store = get_store_fresh();
    let movements = extra_list("inventory_movements.json");
    let product_lookup: HashMap<String, String> = store
        .products
        .iter()
        .map(|p| (p.id.clone(), p.name.clone()))
        .collect();
    let user_lookup: HashMap<String, String> = store
        .users
        .iter()
        .map(|u| (u.id.clone(), u.username.clone()))
        .collect();

    let rows: Vec<(String, String, String, String, String)> = movements
        .iter()
        .map(|item| {
            let product_id = pick_first(item, &["product_id"]);
            let staff_id = pick_first(item, &["staff_id"]);
            let product = product_lookup
                .get(&product_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            let staff = user_lookup
                .get(&staff_id)
                .cloned()
                .unwrap_or_else(|| "Unknown".to_string());
            let qty = pick_first(item, &["quantity"]);
            let date = pick_first(item, &["created_at"]);
            let movement = pick_first(item, &["movement_type"]);
            (product, staff, movement, qty, date)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search stock movements..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Add Brand" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Product" }
                            th { "Person" }
                            th { "Movement" }
                            th { "Qty" }
                            th { "Date" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (product, staff, movement, qty, date) in rows {
                            tr {
                                td { "{product}" }
                                td { "{staff}" }
                                td { "{movement}" }
                                td { "{qty}" }
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
