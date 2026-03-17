use super::prelude::*;

#[component]
pub(crate) fn WarrantiesTab() -> Element {
    let store = get_store_fresh();
    let mut warranties_state = use_signal(|| {
        extra_list("warranties.json")
    });
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut name_state = use_signal(|| String::new());
    let mut description_state = use_signal(|| String::new());
    let mut duration_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = warranties_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                SearchInput { placeholder: "Search warranties..." }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        name_state.set(String::new());
                        description_state.set(String::new());
                        duration_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Warranty" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Warranty" }
                            th { "Description" }
                            th { "Duration" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let id = pick_first(item, &["id", "warranty"]);
                            let name = pick_first(item, &["warranty", "name"]);
                            let description = pick_first(item, &["description"]);
                            let duration = pick_first(item, &["duration"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = id.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                name_state.set(pick_first(&item_clone, &["warranty", "name"]));
                                description_state.set(pick_first(&item_clone, &["description"]));
                                duration_state.set(pick_first(&item_clone, &["duration"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let on_delete = move |_| {
                                let mut next = warranties_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "warranty"]) != id);
                                if save_extra_to_json("warranties.json", &next).is_ok() {
                                    warranties_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{name}" }
                                    td { "{description}" }
                                    td { "{duration}" }
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
                            h3 { "{if edit_id.read().is_some() { \"Edit Warranty\" } else { \"Add Warranty\" }}" }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Warranty" }
                            input { r#type: "text", value: "{name_state.read()}", oninput: move |e| name_state.set(e.value().clone()) }
                        }
                        div { class: "form-group",
                            label { "Description" }
                            input { r#type: "text", value: "{description_state.read()}", oninput: move |e| description_state.set(e.value().clone()) }
                        }
                        div { class: "form-group",
                            label { "Duration" }
                            input { r#type: "text", value: "{duration_state.read()}", oninput: move |e| duration_state.set(e.value().clone()) }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            select { value: "{status_state.read()}", onchange: move |e| status_state.set(e.value().clone()),
                                option { value: "Active", "Active" }
                                option { value: "Inactive", "Inactive" }
                            }
                        }
                        div { class: "form-actions",
                            button { class: "btn-secondary", onclick: move |_| show_modal.set(false), "Cancel" }
                            button { class: "btn-primary", onclick: move |_| {
                                let mut next = warranties_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("W{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "warranty": name_state.read().clone(),
                                    "description": description_state.read().clone(),
                                    "duration": duration_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "warranty"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("warranties.json", &next).is_ok() {
                                    warranties_state.set(next);
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
