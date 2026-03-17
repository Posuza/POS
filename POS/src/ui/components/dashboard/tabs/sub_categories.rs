use super::prelude::*;

#[component]
pub(crate) fn SubCategoriesTab() -> Element {
    let store = get_store_fresh();
    let mut subcats_state = use_signal(|| {
        extra_list("sub_categories.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut category_state = use_signal(|| String::new());
    let mut code_state = use_signal(|| String::new());
    let mut description_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());

    let rows = subcats_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search sub categories..." }
                }
                div { class: "toolbar-actions",
                    button {
                        class: "btn-primary",
                        onclick: move |_| {
                            edit_id.set(None);
                            name_state.set(String::new());
                            category_state.set(String::new());
                            code_state.set(String::new());
                            description_state.set(String::new());
                            status_state.set("Active".to_string());
                            show_modal.set(true);
                        },
                        "Add Sub Category"
                    }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Sub Category" }
                            th { "Category" }
                            th { "Category Code" }
                            th { "Description" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "code"]);
                            let name = pick_first(item, &["sub_category", "name"]);
                            let category = pick_first(item, &["category"]);
                            let code = pick_first(item, &["code", "id"]);
                            let description = pick_first(item, &["description"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["sub_category", "name"]));
                                category_state.set(pick_first(&item_clone, &["category"]));
                                code_state.set(pick_first(&item_clone, &["code", "id"]));
                                description_state.set(pick_first(&item_clone, &["description"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = subcats_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "code"]) != id);
                                if save_extra_to_json("sub_categories.json", &next).is_ok() {
                                    subcats_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{category}" }
                                    td { "{code}" }
                                    td { "{description}" }
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
                            h3 { if edit_id.read().is_some() { "Edit Sub Category" } else { "Add Sub Category" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Sub Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input { r#type: "text", value: "{category_state.read()}", oninput: move |e| category_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Code" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔢" }
                                input { r#type: "text", value: "{code_state.read()}", oninput: move |e| code_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Description" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input { r#type: "text", value: "{description_state.read()}", oninput: move |e| description_state.set(e.value().clone()) }
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
                                let mut next = subcats_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("SC{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "sub_category": name_state.read().clone(),
                                    "category": category_state.read().clone(),
                                    "code": code_state.read().clone(),
                                    "description": description_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "code"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("sub_categories.json", &next).is_ok() {
                                    subcats_state.set(next);
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
