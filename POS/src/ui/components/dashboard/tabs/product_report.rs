use super::prelude::*;

#[component]
pub(crate) fn ProductReportTab() -> Element {
    let store = get_store_fresh();
    let mut category_filter = use_signal(|| "All".to_string());
    let mut brand_filter = use_signal(|| "All".to_string());
    let sale_items = extra_list("sale_items.json");
    let mut totals: HashMap<String, (i32, f32)> = HashMap::new();
    for item in sale_items.iter() {
        let product_id = pick_first(item, &["product_id", "productId", "id"]);
        let qty = value_i32(item, "quantity").unwrap_or(0);
        let total = value_f32(item, "line_total")
            .or_else(|| value_f32(item, "total"))
            .unwrap_or(0.0);
        let entry = totals.entry(product_id).or_insert((0, 0.0));
        entry.0 += qty;
        entry.1 += total;
    }
    let mut categories: Vec<String> = store.products.iter().map(|p| p.category.clone()).collect();
    categories.sort();
    categories.dedup();
    let rows: Vec<(String, String, String, String, String)> = store
        .products
        .iter()
        .filter(|p| {
            let cat_ok =
                category_filter.read().as_str() == "All" || p.category == *category_filter.read();
            let brand_ok = brand_filter.read().as_str() == "All"
                || p.name
                    .to_lowercase()
                    .contains(&brand_filter.read().to_lowercase());
            cat_ok && brand_ok
        })
        .map(|p| {
            let (qty, total) = totals.get(&p.id).cloned().unwrap_or((0, 0.0));
            (
                p.barcode.clone(),
                p.name.clone(),
                p.category.clone(),
                qty.to_string(),
                format_price(total),
            )
        })
        .collect();

    rsx! {
        div { class: "admin-report-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search products..." }
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
                div { class: "form-field",
                    label { "Brand" }
                    select { value: "{brand_filter.read()}", onchange: move |e| brand_filter.set(e.value().clone()),
                        option { value: "All", "All" }
                        option { value: "Lenovo", "Lenovo" }
                        option { value: "Apple", "Apple" }
                        option { value: "Nike", "Nike" }
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
                            th { "Qty" }
                            th { "Revenue" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        for (sku, product, category, qty, revenue) in rows {
                            tr {
                                td { "{sku}" }
                                td { "{product}" }
                                td { "{category}" }
                                td { "{qty}" }
                                td { "{revenue}" }
                                td { RowActions {} }
                            }
                        }
                    }
                }
            }
        }
    }
}
