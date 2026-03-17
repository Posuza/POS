use super::prelude::*;

#[component]
pub(crate) fn BillersTab() -> Element {
    let store = get_store_fresh();
    let mut billers_state = use_signal(|| extra_list("billers.json"));
    let mut show_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut biller_state = use_signal(|| String::new());
    let mut company_state = use_signal(|| String::new());
    let mut email_state = use_signal(|| String::new());
    let mut phone_state = use_signal(|| String::new());
    let mut country_state = use_signal(|| String::new());
    let mut status_state = use_signal(|| "Active".to_string());
    let rows = billers_state.read().clone();
    rsx! {
        div { class: "admin-table-page",
            div { class: "admin-toolbar",
                                div { class: "input-group",
                    span { class: "input-icon-left", "🔍" }
                    input { r#type: "search", placeholder: "Search billers..." }
                }
                div { class: "toolbar-actions",
                    button { class: "btn-primary", onclick: move |_| {
                        edit_id.set(None);
                        biller_state.set(String::new());
                        company_state.set(String::new());
                        email_state.set(String::new());
                        phone_state.set(String::new());
                        country_state.set(String::new());
                        status_state.set("Active".to_string());
                        show_modal.set(true);
                    }, "Add Biller" }
                }
            }
            div { class: "table-container table-compact",
                table { class: "admin-table",
                    thead {
                        tr {
                            th { "Code" }
                            th { "Biller" }
                            th { "Company Name" }
                            th { "Email" }
                            th { "Phone" }
                            th { "Country" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        { rows.iter().map(|item| {
                            let code = pick_first(item, &["id", "code"]);
                            let biller = pick_first(item, &["biller", "name"]);
                            let company = pick_first(item, &["company", "company_name"]);
                            let email = pick_first(item, &["email"]);
                            let phone = pick_first(item, &["phone"]);
                            let country = pick_first(item, &["country"]);
                            let status = pick_first(item, &["status"]);
                            let item_clone = item.clone();
                            let edit_id_clone = code.clone();
                            let on_edit = move |_| {
                                edit_id.set(Some(edit_id_clone.clone()));
                                biller_state.set(pick_first(&item_clone, &["biller", "name"]));
                                company_state.set(pick_first(&item_clone, &["company", "company_name"]));
                                email_state.set(pick_first(&item_clone, &["email"]));
                                phone_state.set(pick_first(&item_clone, &["phone"]));
                                country_state.set(pick_first(&item_clone, &["country"]));
                                status_state.set(pick_first(&item_clone, &["status"]));
                                show_modal.set(true);
                            };
                            let delete_code = code.clone();
                            let on_delete = move |_| {
                                let mut next = billers_state.read().clone();
                                next.retain(|v| pick_first(v, &["id", "code"]) != delete_code);
                                if save_extra_to_json("billers.json", &next).is_ok() {
                                    billers_state.set(next);
                                }
                            };
                            rsx!(
                                tr {
                                    td { "{code}" }
                                    td { "{biller}" }
                                    td { "{company}" }
                                    td { "{email}" }
                                    td { "{phone}" }
                                    td { "{country}" }
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
                            h3 { if edit_id.read().is_some() { "Edit Biller" } else { "Add Biller" } }
                            button { class: "modal-close", onclick: move |_| show_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Biller Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input { r#type: "text", value: "{biller_state.read()}", oninput: move |e| biller_state.set(e.value().clone()) }
                            }
                        }
                        div { class: "form-group",
                            label { "Company" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏢" }
                                input { r#type: "text", value: "{company_state.read()}", oninput: move |e| company_state.set(e.value().clone()) }
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
                            label { "Country" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🌍" }
                                input { r#type: "text", value: "{country_state.read()}", oninput: move |e| country_state.set(e.value().clone()) }
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
                                let mut next = billers_state.read().clone();
                                let id = edit_id.read().clone().unwrap_or_else(|| format!("BL{:03}", next.len() + 1));
                                let value = json!({
                                    "id": id,
                                    "biller": biller_state.read().clone(),
                                    "company": company_state.read().clone(),
                                    "email": email_state.read().clone(),
                                    "phone": phone_state.read().clone(),
                                    "country": country_state.read().clone(),
                                    "status": status_state.read().clone(),
                                });
                                if let Some(edit) = edit_id.read().clone() {
                                    if let Some(entry) = next.iter_mut().find(|v| pick_first(v, &["id", "code"]) == edit) {
                                        *entry = value;
                                    }
                                } else {
                                    next.push(value);
                                }
                                if save_extra_to_json("billers.json", &next).is_ok() {
                                    billers_state.set(next);
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
