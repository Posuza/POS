use super::prelude::*;

#[component]
pub(crate) fn CategoriesTab() -> Element {
    let _store = get_store_fresh();
    let categories = extra_list("categories.json");
    let mut categories_state = use_signal(|| {
        categories
            .iter()
            .map(|c| (*c).clone())
            .collect::<Vec<serde_json::Value>>()
    });
    let _rows = categories_state.read().clone();
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut slug_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let open_new = move |_| {
        edit_id.set(None);
        name_state.set(String::new());
        slug_state.set(String::new());
        status_state.set("Active".to_string());
        show_modal.set(true);
    };

    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search categories..." }
                }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: open_new, "Add Category" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Category" }
                            th { "Category Slug" }
                            th { "Created On" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody { }
                }
            }
            if *show_modal.read() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { if edit_id.read().is_some() { "Edit Category" } else { "Add Category" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Category Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    r#type: "text",
                                    value: "{name_state.read()}",
                                    oninput: move |e| name_state.set(e.value().clone()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Slug" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    r#type: "text",
                                    value: "{slug_state.read()}",
                                    oninput: move |e| slug_state.set(e.value().clone()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select {
                                    value: "{status_state.read()}",
                                    onchange: move |e| status_state.set(e.value().clone()),
                                    option { value: "Active", "Active" }
                                    option { value: "Inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button {
                                class: "btn-primary",
                                onclick: move |_| {
                                    let mut next = categories_state.read().clone();
                                    let id = edit_id.read().clone().unwrap_or_else(|| {
                                        format!("cat{}", next.len() + 1)
                                    });
                                    let value = json!({
                                        "id": id,
                                        "name": name_state.read().clone(),
                                        "slug": slug_state.read().clone(),
                                        "created_at": "2026-01-01",
                                        "status": status_state.read().clone(),
                                    });
                                    if let Some(edit) = edit_id.read().clone() {
                                        if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "slug"]) == edit) {
                                            *entry = value;
                                        }
                                    } else {
                                        next.push(value);
                                    }
                                    if save_extra_to_json("categories.json", &next).is_ok() {
                                        categories_state.set(next);
                                        show_modal.set(false);
                                    }
                                },
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    }
}
