use super::prelude::*;

#[component]
pub(crate) fn SuppliersTab() -> Element {
    let store = get_store_fresh();
    let suppliers = extra_list("suppliers.json");
    let mut suppliers_state = use_signal(|| suppliers.iter().map(|s| (*s).clone()).collect::<Vec<_>>());
    let suppliers_owned = suppliers_state.read().clone();
    let active_suppliers = suppliers_owned.iter().filter(|s| {
        let status = pick_first(s, &["status"]);
        status.is_empty() || status.eq_ignore_ascii_case("active")
    }).count();
    let mut add_name = use_signal(|| String::new());
    let mut add_contact = use_signal(|| String::new());
    let mut add_phone = use_signal(|| String::new());
    let mut add_email = use_signal(|| String::new());
    let mut add_address = use_signal(|| String::new());
    let mut add_status = use_signal(|| "active".to_string());
    let mut add_msg = use_signal(|| None::<String>);
    let mut add_error = use_signal(|| None::<String>);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_contact = use_signal(|| String::new());
    let mut edit_phone = use_signal(|| String::new());
    let mut edit_email = use_signal(|| String::new());
    let mut edit_address = use_signal(|| String::new());
    let mut edit_status = use_signal(|| "active".to_string());
    let mut edit_msg = use_signal(|| None::<String>);
    let mut edit_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);

    let handle_add_supplier = {
        let mut suppliers_state = suppliers_state.clone();
        let mut add_name = add_name.clone();
        let mut add_contact = add_contact.clone();
        let mut add_phone = add_phone.clone();
        let mut add_email = add_email.clone();
        let mut add_address = add_address.clone();
        let mut add_status = add_status.clone();
        let mut add_msg = add_msg.clone();
        let mut add_error = add_error.clone();
        move |_| {
            if add_name.read().is_empty() {
                add_error.set(Some("Supplier name is required.".to_string()));
                return;
            }
            if !is_valid_email(&add_email.read()) {
                add_error.set(Some("Please enter a valid email address.".to_string()));
                return;
            }
            let mut updated = suppliers_state.read().clone();
            let now = now_iso();
            updated.push(json!({
                "id": new_id("sup"),
                "name": add_name.read().clone(),
                "contact_name": add_contact.read().clone(),
                "phone": add_phone.read().clone(),
                "email": add_email.read().clone(),
                "address": add_address.read().clone(),
                "status": add_status.read().clone(),
                "created_at": now,
                "updated_at": now,
            }));
            match save_extra_to_json("suppliers.json", &updated) {
                Ok(_) => {
                    suppliers_state.set(updated);
                    add_error.set(None);
                    add_msg.set(Some("✅ Supplier added.".to_string()));
                    add_name.set(String::new());
                    add_contact.set(String::new());
                    add_phone.set(String::new());
                    add_email.set(String::new());
                    add_address.set(String::new());
                    add_status.set("active".to_string());
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_msg.set(None);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };

    let handle_edit_supplier = {
        let mut suppliers_state = suppliers_state.clone();
        let mut edit_id = edit_id.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_contact = edit_contact.clone();
        let mut edit_phone = edit_phone.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_address = edit_address.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_msg = edit_msg.clone();
        let mut edit_error = edit_error.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        move |_| {
            let id = match edit_id.read().clone() {
                Some(id) => id,
                None => return,
            };
            let mut updated = suppliers_state.read().clone();
            if let Some(entry) = updated.iter_mut().find(|s| pick_first(s, &["id", "supplier_id"]) == id) {
                if let Some(obj) = entry.as_object_mut() {
                    if !is_valid_email(&edit_email.read()) {
                        edit_error.set(Some("Please enter a valid email address.".to_string()));
                        return;
                    }
                    obj.insert("name".to_string(), json!(edit_name.read().clone()));
                    obj.insert("contact_name".to_string(), json!(edit_contact.read().clone()));
                    obj.insert("phone".to_string(), json!(edit_phone.read().clone()));
                    obj.insert("email".to_string(), json!(edit_email.read().clone()));
                    obj.insert("address".to_string(), json!(edit_address.read().clone()));
                    obj.insert("status".to_string(), json!(edit_status.read().clone()));
                    obj.insert("updated_at".to_string(), json!(now_iso()));
                }
            } else {
                edit_error.set(Some("Supplier not found.".to_string()));
                return;
            }
            match save_extra_to_json("suppliers.json", &updated) {
                Ok(_) => {
                    suppliers_state.set(updated);
                    edit_error.set(None);
                    edit_msg.set(Some("✅ Supplier updated.".to_string()));
                    show_edit_modal.set(false);
                    edit_id.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        edit_msg.set(None);
                    });
                }
                Err(err) => edit_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "content-card",
            h2 { "🚚 Suppliers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {suppliers_owned.len()}" }
                div { class: "ops-pill success", "Active: {active_suppliers}" }
            }

            if let Some(msg) = add_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = add_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = edit_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = edit_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = delete_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = delete_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Add Supplier" }
                    span { class: "card-subtitle", "Create a new supplier profile." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Supplier Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏭" }
                            input {
                                placeholder: "Fresh Farms Ltd",
                                value: "{add_name}",
                                oninput: move |e| add_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Contact Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "👤" }
                            input {
                                placeholder: "A. Rivera",
                                value: "{add_contact}",
                                oninput: move |e| add_contact.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0101",
                                value: "{add_phone}",
                                oninput: move |e| add_phone.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "orders@freshfarms.local",
                                value: "{add_email}",
                                oninput: move |e| add_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Address" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📍" }
                            input {
                                placeholder: "12 Market St",
                                value: "{add_address}",
                                oninput: move |e| add_address.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Status" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{add_status}",
                                onchange: move |e| add_status.set(e.value()),
                                option { value: "active", "Active" }
                                option { value: "inactive", "Inactive" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_add_supplier,
                        "Add Supplier"
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Supplier Directory" }
                    span { class: "card-subtitle", "{suppliers_owned.len()} records" }
                }
                div { class: "admin-card-body",
                    div { class: "table-container",
                        table { class: "admin-table",
                            thead {
                                tr {
                                    th { "Supplier" }
                                    th { "Contact" }
                                    th { "Phone" }
                                    th { "Email" }
                                    th { "Status" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                if suppliers_owned.is_empty() {
                                    tr { td { colspan: "6",
                                        div { class: "empty-state", "No suppliers available." }
                                    } }
                                } else {
                                    { suppliers_owned.iter().map(|s| {
                                        let sid = pick_first(s, &["id", "supplier_id"]);
                                        let name = pick_first(s, &["name", "company", "supplier"]);
                                        let contact = pick_first(s, &["contact_name", "contact", "person"]);
                                        let phone = pick_first(s, &["phone"]);
                                        let email = pick_first(s, &["email"]);
                                        let status = pick_first(s, &["status", "tier", "rating"]);
                                        let address = pick_first(s, &["address"]);
                                        let mut show_edit_modal = show_edit_modal.clone();
                                        let mut edit_id = edit_id.clone();
                                        let mut edit_name = edit_name.clone();
                                        let mut edit_contact = edit_contact.clone();
                                        let mut edit_phone = edit_phone.clone();
                                        let mut edit_email = edit_email.clone();
                                        let mut edit_address = edit_address.clone();
                                        let mut edit_status = edit_status.clone();
                                        let mut suppliers_state = suppliers_state.clone();
                                        let mut delete_msg = delete_msg.clone();
                                        let mut delete_error = delete_error.clone();
                                        let sid_for_delete = sid.clone();
                                        rsx!(
                                            tr {
                                                td { "{name}" }
                                                td { "{contact}" }
                                                td { "{phone}" }
                                                td { "{email}" }
                                                td { span { class: "data-chip", "{status}" } }
                                                td { div { class: "table-actions",
                                                    button {
                                                        class: "btn-small",
                                                        onclick: move |_| {
                                                            edit_id.set(Some(sid.clone()));
                                                            edit_name.set(name.clone());
                                                            edit_contact.set(contact.clone());
                                                            edit_phone.set(phone.clone());
                                                            edit_email.set(email.clone());
                                                            edit_address.set(address.clone());
                                                            edit_status.set(if status.is_empty() { "active".to_string() } else { status.clone() });
                                                            show_edit_modal.set(true);
                                                        },
                                                        "✏️ Edit"
                                                    }
                                                    button {
                                                        class: "btn-small btn-danger",
                                                        onclick: move |_| {
                                                            let mut updated = suppliers_state.read().clone();
                                                            updated.retain(|s| pick_first(s, &["id", "supplier_id"]) != sid_for_delete);
                                                            match save_extra_to_json("suppliers.json", &updated) {
                                                                Ok(_) => {
                                                                    suppliers_state.set(updated);
                                                                    delete_error.set(None);
                                                                    delete_msg.set(Some("✅ Supplier deleted.".to_string()));
                                                                    spawn(async move {
                                                                        sleep_ms(2_000).await;
                                                                        delete_msg.set(None);
                                                                    });
                                                                }
                                                                Err(err) => delete_error.set(Some(err)),
                                                            }
                                                        },
                                                        "🗑️ Delete"
                                                    }
                                                } }
                                            }
                                        )
                                    }) }
                                }
                            }
                        }
                    }
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Supplier" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }
                        div { class: "form-group",
                            label { "Supplier Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏭" }
                                input {
                                    value: "{edit_name}",
                                    oninput: move |e| edit_name.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Contact Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "👤" }
                                input {
                                    value: "{edit_contact}",
                                    oninput: move |e| edit_contact.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Phone" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📞" }
                                input {
                                    value: "{edit_phone}",
                                    oninput: move |e| edit_phone.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Email" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "✉️" }
                                input {
                                    value: "{edit_email}",
                                    oninput: move |e| edit_email.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Address" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📍" }
                                input {
                                    value: "{edit_address}",
                                    oninput: move |e| edit_address.set(e.value()),
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Status" }
                            div { class: "input-group input-right",
                                span { class: "input-icon-right", "▾" }
                                select {
                                    value: "{edit_status}",
                                    onchange: move |e| edit_status.set(e.value()),
                                    option { value: "active", "Active" }
                                    option { value: "inactive", "Inactive" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_supplier,
                                "Save Changes"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(false),
                                "Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}
