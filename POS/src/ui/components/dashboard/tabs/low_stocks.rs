use super::prelude::*;

#[component]
pub(crate) fn LowStocksTab() -> Element {
    let store = get_store_fresh();
    let low_products: Vec<(String, String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .map(|p| {
            let status = if p.quantity == 0 { "Out" } else { "Low" };
            (
                p.barcode.clone(),
                p.name.clone(),
                p.category.clone(),
                p.quantity.to_string(),
                status.to_string(),
            )
        })
        .collect();

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search low stock..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Send Email" }
                    button { class: "btn-secondary", "Notify" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Category" }
                            th { "Qty" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, name, category, qty, status) in low_products {
                            tr {
                                td { "{sku}" }
                                td { "{name}" }
                                td { "{category}" }
                                td { "{qty}" }
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
