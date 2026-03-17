use super::prelude::*;

#[component]
pub(crate) fn WarehousesTab() -> Element {
    let store = get_store_fresh();
    let mut warehouses_state = use_signal(|| {
        extra_list("warehouses.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut contact_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut total_products_state = use_signal(|| String::from("0"));
    let mut stock_state = use_signal(|| String::from("0"));
    let mut qty_state = use_signal(|| String::from("0"));
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = warehouses_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search warehouse..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        contact_state.set(String::new());
                        phone_state.set(String::new());
                        total_products_state.set("0".to_string());
                        stock_state.set("0".to_string());
                        qty_state.set("0".to_string());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Warehouse" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Warehouse" }
                            th { "Contact Person" }
                            th { "Phone" }
                            th { "Total Products" }
                            th { "Stock" }
                            th { "Qty" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "warehouse"]);
                            let warehouse = pick_first(item, &["warehouse", "name"]);
                            let contact = pick_first(item, &["contact", "contact_person"]);
                            let phone = pick_first(item, &["phone"]);
                            let total_products = pick_first(item, &["total_products"]);
                            let stock = pick_first(item, &["stock"]);
                            let qty = pick_first(item, &["qty", "quantity"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["warehouse", "name"]));
                                contact_state.set(pick_first(&item_clone, &["contact", "contact_person"]));
                                phone_state.set(pick_first(&item_clone, &["phone"]));
                                total_products_state.set(pick_first(&item_clone, &["total_products"]));
                                stock_state.set(pick_first(&item_clone, &["stock"]));
                                qty_state.set(pick_first(&item_clone, &["qty", "quantity"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = warehouses_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "warehouse"]) != id);
                                if save_extra_to_json("warehouses.json", &next).is_ok() {
                                    warehouses_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{warehouse}" }
                                    td { "{contact}" }
                                    td { "{phone}" }
                                    td { "{total_products}" }
                                    td { "{stock}" }
                                    td { "{qty}" }
                                    td { "{created}" }
                                    td { StatusChip { label: status } }
                                    td {
                                        div { class: "table-actions",
                                            button { class: "btn-secondary", onclick: on_edit, "Edit" }
                                            button { class: "btn-danger", onclick: on_delete, "Delete" }
                                        }
                                    }
                                }
                            )
                        }) }
                    }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "{if edit_id.read().is_some() { \"Edit Warehouse\" } else { \"Add Warehouse\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Warehouse" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏬" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Contact Person" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{contact_state.read()}", oninput: move |e| contact_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input { r#type: "text", value: "{phone_state.read()}", oninput: move |e| phone_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Total Products" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "number", value: "{total_products_state.read()}", oninput: move |e| total_products_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "number", value: "{stock_state.read()}", oninput: move |e| stock_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Qty" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "number", value: "{qty_state.read()}", oninput: move |e| qty_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value().clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = warehouses_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("wh{}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "warehouse": name_state.read().clone(),
                                    "contact": contact_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "total_products": total_products_state.read().clone(),
                                    "stock": stock_state.read().clone(),
                                    "qty": qty_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "warehouse"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("warehouses.json", &next).is_ok() {
                                    warehouses_state.set(next);
                                    show_modal.set(false);
                                }
                            }, "Save" }
                        }
                    }
                }
            }
        }
    }
}
