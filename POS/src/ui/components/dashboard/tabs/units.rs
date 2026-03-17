use super::prelude::*;

#[component]
pub(crate) fn UnitsTab() -> Element {
    let store = get_store_fresh();
    let mut units_state = use_signal(|| {
        extra_list("units.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut short_state = use_signal(|| String::new());
    let mut products_state = use_signal(|| String::from("0"));
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = units_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search units..." }
                }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        short_state.set(String::new());
                        products_state.set("0".to_string());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Unit" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Unit" }
                            th { "Short Name" }
                            th { "No of Products" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "unit"]);
                            let name = pick_first(item, &["unit", "name"]);
                            let short = pick_first(item, &["short", "abbr"]);
                            let products = pick_first(item, &["products", "count"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["unit", "name"]));
                                short_state.set(pick_first(&item_clone, &["short", "abbr"]));
                                products_state.set(pick_first(&item_clone, &["products", "count"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = units_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "unit"]) != id);
                                if save_extra_to_json("units.json", &next).is_ok() {
                                    units_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{short}" }
                                    td { "{products}" }
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
                            h3 { if edit_id.read().is_some() { "Edit Unit" } else { "Add Unit" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Unit Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Short Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔤" }
                                input { r#type: "text", value: "{short_state.read()}", oninput: move |e| short_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "No of Products" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "number", value: "{products_state.read()}", oninput: move |e| products_state.set(e.value().clone()) }
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
                                let mut next = units_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("U{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "unit": name_state.read().clone(),
                                    "short": short_state.read().clone(),
                                    "products": products_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "unit"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("units.json", &next).is_ok() {
                                    units_state.set(next);
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
