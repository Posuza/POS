use super::prelude::*;

#[component]
pub(crate) fn StoresTab() -> Element {
    let _store = get_store_fresh();
    let mut stores_state = use_signal(|| extra_list("stores.json"));
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut user_state = use_signal(|| String::new());
    let mut email_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = stores_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search stores..." }
                }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        user_state.set(String::new());
                        email_state.set(String::new());
                        phone_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Store" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Store" }
                            th { "User Name" }
                            th { "Email" }
                            th { "Phone" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "store"]);
                            let store_name = pick_first(item, &["store", "name"]);
                            let user = pick_first(item, &["user", "username"]);
                            let email = pick_first(item, &["email"]);
                            let phone = pick_first(item, &["phone"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["store", "name"]));
                                user_state.set(pick_first(&item_clone, &["user", "username"]));
                                email_state.set(pick_first(&item_clone, &["email"]));
                                phone_state.set(pick_first(&item_clone, &["phone"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = stores_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "store"]) != id);
                                if save_extra_to_json("stores.json", &next).is_ok() {
                                    stores_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{store_name}" }
                                    td { "{user}" }
                                    td { "{email}" }
                                    td { "{phone}" }
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
                            h3 { if edit_id.read().is_some() { "Edit Store" } else { "Add Store" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Store Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏬" }
                                input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "User Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{user_state.read()}", oninput: move |e| user_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input { r#type: "email", value: "{email_state.read()}", oninput: move |e| email_state.set(e.value().clone()) }
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
                                let mut next = stores_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("st{}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "store": name_state.read().clone(),
                                    "user": user_state.read().clone(),
                                    "email": email_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "store"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("stores.json", &next).is_ok() {
                                    stores_state.set(next);
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
