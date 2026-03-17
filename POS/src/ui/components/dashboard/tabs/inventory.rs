use super::prelude::*;

#[component]
pub(crate) fn InventoryTab() -> Element {
    let store = get_store_fresh();
    let products = &store.products;
    let movements = extra_list("inventory_movements.json");
    let categories = extra_list("categories.json");
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock = products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .count();
    let total_value: f32 = products.iter().map(|p| p.price * p.quantity as f32).sum();
    let mut category_values: HashMap<String, f32> = HashMap::new();
    for p in products.iter() {
        *category_values.entry(p.category.clone()).or_insert(0.0) += p.price * p.quantity as f32;
    }
    let mut category_value_vec: Vec<(String, f32)> = category_values.into_iter().collect();
    category_value_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut reorder_list: Vec<&crate::data::models::product::Product> = products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .collect();
    reorder_list.sort_by(|a, b| a.quantity.cmp(&b.quantity));

    rsx! {
        div { class: "content-card",
            h2 { "🗄️ Inventory" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Products: {products.len()}" }
                div { class: "ops-pill warning", "Threshold: {LOW_STOCK_THRESHOLD}" }
                div { class: "ops-pill warning", "Low stock: {low_stock}" }
                div { class: "ops-pill danger", "Out: {out_of_stock}" }
                div { class: "ops-pill", "Movements: {movements.len()}" }
                div { class: "ops-pill", "Value: {format_price(total_value)}" }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Movements" }
                        span { class: "card-note", "{movements.len()} logs" }
                    }
                    div { class: "data-list",
                        if movements.is_empty() {
                            div { class: "empty-state", "No inventory movements yet." }
                        } else {
                            { movements.iter().rev().take(5).map(|m| {
                                let item = pick_first(m, &["product_name", "product_id", "sku", "id"]);
                                let qty = pick_first(m, &["quantity", "qty", "delta"]);
                                let kind = pick_first(m, &["type", "reason", "action"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{item}" }
                                            span { "{kind}" }
                                        }
                                        span { class: "data-chip", "{qty}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Categories" }
                        span { class: "card-note", "{categories.len()} groups" }
                    }
                    div { class: "data-list",
                        if categories.is_empty() {
                            div { class: "empty-state", "No categories configured." }
                        } else {
                            { categories.iter().take(6).map(|c| {
                                let name = pick_first(c, &["name", "title", "category"]);
                                let code = pick_first(c, &["code", "slug", "id"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "Code: {code}" }
                                        }
                                        span { class: "data-chip", "Active" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Category Valuation" }
                        span { class: "card-note", "{category_value_vec.len()} categories" }
                    }
                    div { class: "data-list",
                        if category_value_vec.is_empty() {
                            div { class: "empty-state", "No valuation data yet." }
                        } else {
                            { category_value_vec.iter().take(6).map(|(name, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "Inventory value" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Reorder Insights" }
                        span { class: "card-note", "{reorder_list.len()} items" }
                    }
                    div { class: "data-list",
                        if reorder_list.is_empty() {
                            div { class: "empty-state", "No low-stock items." }
                        } else {
                            { reorder_list.iter().take(6).map(|p| {
                                let status = if p.quantity == 0 { "Out" } else { "Low" };
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{p.name}" }
                                            span { "{p.category}" }
                                        }
                                        span { class: "data-chip", "{status}: {p.quantity}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }
        }
    }
}
