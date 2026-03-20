use super::prelude::*;

#[component]
pub(crate) fn BrandsTab() -> Element {
    let _store = get_store_fresh();
    let mut brands_state = use_signal(|| extra_list("brands.json"));
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = brands_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search brands..." }
                }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Brand" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Brand" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "brand"]);
                            let name = pick_first(item, &["brand", "name"]);
                            let created = pick_first(item, &["created_at", "created"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["brand", "name"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = brands_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "brand"]) != id);
                                if save_extra_to_json("brands.json", &next).is_ok() {
                                    brands_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
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
                            h3 { if edit_id.read().is_some() { "Edit Brand" } else { "Add Brand" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Brand Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
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
                                let mut next = brands_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("BR{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "brand": name_state.read().clone(),
                                    "created_at": "2026-01-01",
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "brand"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("brands.json", &next).is_ok() {
                                    brands_state.set(next);
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
