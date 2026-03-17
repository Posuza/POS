use super::prelude::*;

#[component]
pub(crate) fn ExpiredProductsTab() -> Element {
    let store = get_store_fresh();
    let expired = extra_list("expired_products.json");
    let rows: Vec<(String, String, String, String, String)> = expired
        .iter()
        .map(|item| {
            let sku = pick_first(item, &["sku", "code"]);
            let product = pick_first(item, &["product", "name"]);
            let manufactured = pick_first(item, &["manufactured", "manufactured_at"]);
            let expired_at = pick_first(item, &["expired", "expired_at"]);
            let status = "Expired".to_string();
            (sku, product, manufactured, expired_at, status)
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search expired products..." }
                div { class: "toolbar-actions",
                    button { class: "btn-secondary", "Export" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Manufactured" }
                            th { "Expired" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, manufactured, expired_at, status) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{manufactured}" }
                                td { "{expired_at}" }
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
