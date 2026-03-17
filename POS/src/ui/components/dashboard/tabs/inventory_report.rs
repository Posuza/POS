use super::prelude::*;

#[component]
pub(crate) fn InventoryReportTab() -> Element {
    let store = get_store_fresh();
    let mut category_filter = use_signal(|| "All".to_string());
    let mut categories: Vec<String> = store.products.iter().map(|p| p.category.clone()).collect();
    categories.sort();
    categories.dedup();
    let rows: Vec<(String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| {
            category_filter.read().as_str() == "All" || p.category == *category_filter.read()
        })
        .map(|p| {
            (
                p.barcode.clone(),
                p.name.clone(),
                p.category.clone(),
                p.quantity.to_string(),
            )
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search inventory..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", "Generate Report" }
                }
            }
            div { class: "report-filters",
                div { class: "form-field",
                    label { "Category" }
                    select { value: "{category_filter.read()}", onchange: move |e| category_filter.set(e.value().clone()),
                        option { value: "All", "All" }
                        for cat in categories.iter() {
                            option { value: "{cat}", "{cat}" }
                        }
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "SKU" }
                            th { "Product Name" }
                            th { "Category" }
                            th { "In Stock" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, category, qty) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{category}" }
                                td { "{qty}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}
