use super::prelude::*;

#[component]
pub(crate) fn CustomersTab() -> Element {
    let store = get_store_fresh();
    let customers = extra_list("customers.json");
    let mut customers_state =
        use_signal(|| customers.iter().map(|c| (*c).clone()).collect::<Vec<_>>());
    let customers_owned = customers_state.read().clone();
    let active_customers = customers_owned
        .iter()
        .filter(|c| {
            let status = pick_first(c, &["status"]);
            status.is_empty() || status.eq_ignore_ascii_case("active")
        })
        .count();
    let sales = &store.sales;
    let mut customer_export_msg = use_signal(|| None::<String>);
    let mut customer_export_error = use_signal(|| None::<String>);
    let mut customer_query = use_signal(|| String::new());
    let mut customer_sort = use_signal(|| "name".to_string());
    let mut customer_order = use_signal(|| "asc".to_string());
    let mut customer_page = use_signal(|| 1usize);
    let mut add_name = use_signal(|| String::new());
    let mut add_email = use_signal(|| String::new());
    let mut add_phone = use_signal(|| String::new());
    let mut add_status = use_signal(|| "active".to_string());
    let mut add_image_payload = use_signal(|| None::<String>);
    let mut add_image_type = use_signal(|| None::<String>);
    let mut add_image_preview = use_signal(|| None::<String>);
    let mut add_msg = use_signal(|| None::<String>);
    let mut add_error = use_signal(|| None::<String>);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_id = use_signal(|| None::<String>);
    let mut edit_name = use_signal(|| String::new());
    let mut edit_email = use_signal(|| String::new());
    let mut edit_phone = use_signal(|| String::new());
    let mut edit_status = use_signal(|| "active".to_string());
    let mut edit_image_payload = use_signal(|| None::<String>);
    let mut edit_image_type = use_signal(|| None::<String>);
    let mut edit_image_name = use_signal(|| None::<String>);
    let mut edit_image_preview = use_signal(|| None::<String>);
    let mut edit_msg = use_signal(|| None::<String>);
    let mut edit_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);

    let handle_add_customer = {
        let mut customers_state = customers_state.clone();
        let mut add_name = add_name.clone();
        let mut add_email = add_email.clone();
        let mut add_phone = add_phone.clone();
        let mut add_status = add_status.clone();
        let mut add_image_payload = add_image_payload.clone();
        let mut add_image_type = add_image_type.clone();
        let mut add_image_preview = add_image_preview.clone();
        let mut add_msg = add_msg.clone();
        let mut add_error = add_error.clone();
        move |_| {
            if add_name.read().is_empty() {
                add_error.set(Some("Customer name is required.".to_string()));
                return;
            }
            if !is_valid_email(&add_email.read()) {
                add_error.set(Some("Please enter a valid email address.".to_string()));
                return;
            }
            let mut updated = customers_state.read().clone();
            let id = new_id("cust");
            let mut image_filename: Option<String> = None;
            let mut image_type_saved: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (
                add_image_payload.read().clone(),
                add_image_type.read().clone(),
            ) {
                match ImageService::save_user_image(&payload, &img_type, &id) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        add_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            updated.push(json!({
                "id": id,
                "name": add_name.read().clone(),
                "email": add_email.read().clone(),
                "phone": add_phone.read().clone(),
                "status": add_status.read().clone(),
                "loyalty_points": 0,
                "profile_image": image_filename,
                "profile_image_type": image_type_saved,
                "created_at": now,
                "updated_at": now,
            }));
            match save_extra_to_json("customers.json", &updated) {
                Ok(_) => {
                    customers_state.set(updated);
                    add_error.set(None);
                    add_msg.set(Some("✅ Customer added.".to_string()));
                    add_name.set(String::new());
                    add_email.set(String::new());
                    add_phone.set(String::new());
                    add_status.set("active".to_string());
                    add_image_payload.set(None);
                    add_image_type.set(None);
                    add_image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_msg.set(None);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };

    let handle_edit_customer = {
        let mut customers_state = customers_state.clone();
        let mut edit_id = edit_id.clone();
        let mut edit_name = edit_name.clone();
        let mut edit_email = edit_email.clone();
        let mut edit_phone = edit_phone.clone();
        let mut edit_status = edit_status.clone();
        let mut edit_image_payload = edit_image_payload.clone();
        let mut edit_image_type = edit_image_type.clone();
        let mut edit_image_name = edit_image_name.clone();
        let mut edit_image_preview = edit_image_preview.clone();
        let mut edit_msg = edit_msg.clone();
        let mut edit_error = edit_error.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        move |_| {
            let id = match edit_id.read().clone() {
                Some(id) => id,
                None => return,
            };
            let mut updated = customers_state.read().clone();
            let mut image_filename = edit_image_name.read().clone();
            let mut image_type_saved = edit_image_type.read().clone();
            if let (Some(payload), Some(img_type)) = (
                edit_image_payload.read().clone(),
                edit_image_type.read().clone(),
            ) {
                match ImageService::save_user_image(&payload, &img_type, &id) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        edit_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            if let Some(entry) = updated
                .iter_mut()
                .find(|c| pick_first(c, &["id", "customer_id"]) == id)
            {
                if let Some(obj) = entry.as_object_mut() {
                    if !is_valid_email(&edit_email.read()) {
                        edit_error.set(Some("Please enter a valid email address.".to_string()));
                        return;
                    }
                    obj.insert("name".to_string(), json!(edit_name.read().clone()));
                    obj.insert("email".to_string(), json!(edit_email.read().clone()));
                    obj.insert("phone".to_string(), json!(edit_phone.read().clone()));
                    obj.insert("status".to_string(), json!(edit_status.read().clone()));
                    obj.insert("profile_image".to_string(), json!(image_filename));
                    obj.insert("profile_image_type".to_string(), json!(image_type_saved));
                    obj.insert("updated_at".to_string(), json!(now_iso()));
                }
            } else {
                edit_error.set(Some("Customer not found.".to_string()));
                return;
            }
            match save_extra_to_json("customers.json", &updated) {
                Ok(_) => {
                    customers_state.set(updated);
                    edit_error.set(None);
                    edit_msg.set(Some("✅ Customer updated.".to_string()));
                    show_edit_modal.set(false);
                    edit_id.set(None);
                    edit_image_payload.set(None);
                    edit_image_type.set(None);
                    edit_image_name.set(None);
                    edit_image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        edit_msg.set(None);
                    });
                }
                Err(err) => edit_error.set(Some(err)),
            }
        }
    };

    let query = customer_query.read().to_lowercase();
    let mut filtered_customers: Vec<serde_json::Value> = customers_owned
        .clone()
        .into_iter()
        .filter(|c| {
            if query.is_empty() {
                true
            } else {
                let name = pick_first(c, &["name", "full_name", "username", "id"]).to_lowercase();
                let email = pick_first(c, &["email", "contact_email", "phone"]).to_lowercase();
                let tier = pick_first(c, &["tier", "segment", "status"]).to_lowercase();
                name.contains(&query) || email.contains(&query) || tier.contains(&query)
            }
        })
        .collect();
    let sort_key = customer_sort.read().clone();
    filtered_customers.sort_by(|a, b| {
        let a_val = pick_first(a, &["name", "full_name", "username", "id"]).to_lowercase();
        let b_val = pick_first(b, &["name", "full_name", "username", "id"]).to_lowercase();
        match sort_key.as_str() {
            "status" => pick_first(a, &["status", "tier", "segment"])
                .to_lowercase()
                .cmp(&pick_first(b, &["status", "tier", "segment"]).to_lowercase()),
            _ => a_val.cmp(&b_val),
        }
    });
    if customer_order.read().as_str() == "desc" {
        filtered_customers.reverse();
    }
    let page_size = 8usize;
    let total_pages = std::cmp::max(1, (filtered_customers.len() + page_size - 1) / page_size);
    let current_page = (*customer_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_customers.len());
    let page_items: Vec<serde_json::Value> = if filtered_customers.is_empty() {
        Vec::new()
    } else {
        filtered_customers[start..end].to_vec()
    };
    let mut customer_rows: Vec<Element> = Vec::new();
    for c in page_items.clone().into_iter() {
        let name = pick_first(&c, &["name", "full_name", "username", "id"]);
        let email = pick_first(&c, &["email", "contact_email", "phone"]);
        let tier = pick_first(&c, &["tier", "segment", "status"]);
        customer_rows.push(rsx!(
            div { class: "data-row",
                div { class: "data-main",
                    strong { "{name}" }
                    span { "{email}" }
                }
                span { class: "data-chip", "{tier}" }
            }
        ));
    }
    let mut customer_sales: HashMap<String, i32> = HashMap::new();
    for s in sales.iter() {
        if let Some(cid) = s.customer_id.clone() {
            *customer_sales.entry(cid).or_insert(0) += 1;
        }
    }
    let repeat_customers = customer_sales.values().filter(|c| **c > 1).count();
    let unique_customers = customer_sales.len();
    let repeat_rate = if unique_customers == 0 {
        0.0
    } else {
        (repeat_customers as f32 / unique_customers as f32) * 100.0
    };
    let repeat_rate_s = format!("{:.1}%", repeat_rate);

    rsx! {
        div { class: "content-card",
            h2 { "🧑‍🤝‍🧑 Customers" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {customers_owned.len()}" }
                div { class: "ops-pill success", "Active: {active_customers}" }
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
                    h3 { "Add Customer" }
                    span { class: "card-subtitle", "Create a new customer profile." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Customer Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            input {
                                placeholder: "Jane Doe",
                                value: "{add_name}",
                                oninput: move |e| add_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "jane@example.com",
                                value: "{add_email}",
                                oninput: move |e| add_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0200",
                                value: "{add_phone}",
                                oninput: move |e| add_phone.set(e.value()),
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
                    div { class: "form-field span-2",
                        label { "Profile Image" }
                        div { class: "file-input-wrapper",
                            button { class: "btn btn-secondary", "📷 Choose Image" }
                            { let mut add_image_payload = add_image_payload.clone();
                              let mut add_image_type = add_image_type.clone();
                              let mut add_image_preview = add_image_preview.clone();
                              let mut add_error = add_error.clone();
                              rsx!(
                                input {
                                    r#type: "file",
                                    accept: "image/*",
                                    onchange: move |e| {
                                        let path = e.value();
                                        if path.is_empty() {
                                            return;
                                        }
                                        match load_image_from_path(&path) {
                                            Ok((payload, img_type, preview)) => {
                                                add_image_payload.set(Some(payload));
                                                add_image_type.set(Some(img_type));
                                                add_image_preview.set(Some(preview));
                                            }
                                            Err(err) => add_error.set(Some(format!("Image load failed: {}", err))),
                                        }
                                    },
                                }
                              )
                            }
                        }
                        if let Some(img) = add_image_preview.read().clone() {
                            div { class: "image-preview",
                                img { src: "{img}" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_add_customer,
                        "Add Customer"
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "filter-group filter-inline",
                    label { "Search" }
                    input {
                        placeholder: "Name, email, segment",
                        value: "{customer_query}",
                        oninput: move |e| {
                            customer_query.set(e.value());
                            customer_page.set(1);
                        },
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Sort" }
                    select {
                        value: "{customer_sort}",
                        onchange: move |e| customer_sort.set(e.value()),
                        option { value: "name", "Name" }
                        option { value: "status", "Status" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Order" }
                    select {
                        value: "{customer_order}",
                        onchange: move |e| customer_order.set(e.value()),
                        option { value: "asc", "Asc" }
                        option { value: "desc", "Desc" }
                    }
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_customers_csv(&customers_owned) {
                            Ok(path) => {
                                customer_export_error.set(None);
                                customer_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => customer_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
            }

            div { class: "data-ops-grid",
                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Recent Customers" }
                        span { class: "card-note", "{customers_owned.len()} records" }
                    }
                    div { class: "data-list",
                        if customer_rows.is_empty() {
                            div { class: "empty-state", "No customer data found." }
                        } else {
                            { customer_rows.into_iter() }
                        }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Customer Insights" }
                        span { class: "card-note", "Engagement" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Active" }
                            strong { "{active_customers}" }
                        }
                        div { class: "data-stat",
                            span { "Inactive" }
                            strong { "{customers_owned.len().saturating_sub(active_customers)}" }
                        }
                        div { class: "data-stat",
                            span { "Records" }
                            strong { "{customers_owned.len()}" }
                        }
                    }
                    div { class: "data-list",
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Unique customers" }
                                span { "From sales history" }
                            }
                            span { class: "data-chip", "{unique_customers}" }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Repeat customers" }
                                span { "2+ purchases" }
                            }
                            span { class: "data-chip", "{repeat_customers}" }
                        }
                        div { class: "data-row",
                            div { class: "data-main",
                                strong { "Repeat rate" }
                                span { "Repeat / unique" }
                            }
                            span { class: "data-chip", "{repeat_rate_s}" }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Customer Directory" }
                    span { class: "card-subtitle", "Manage customer records." }
                }
                div { class: "admin-card-body",
                    div { class: "table-container",
                        table { class: "admin-table",
                            thead {
                                tr {
                                    th { "Name" }
                                    th { "Email" }
                                    th { "Phone" }
                                    th { "Status" }
                                    th { "Actions" }
                                }
                            }
                            tbody {
                                if filtered_customers.is_empty() {
                                    tr { td { colspan: "5",
                                        div { class: "empty-state", "No customers match your filters." }
                                    } }
                                } else {
                                    { filtered_customers.iter().map(|c| {
                                        let cid = pick_first(c, &["id", "customer_id"]);
                                        let name = pick_first(c, &["name", "full_name", "username", "id"]);
                                        let email = pick_first(c, &["email", "contact_email", "phone"]);
                                        let phone = pick_first(c, &["phone", "contact_phone", "mobile"]);
                                        let status = pick_first(c, &["status", "tier", "segment"]);
                                        let image_name = pick_first(c, &["profile_image", "image"]);
                                        let image_type = pick_first(c, &["profile_image_type", "image_type"]);
                                        let mut show_edit_modal = show_edit_modal.clone();
                                        let mut edit_id = edit_id.clone();
                                        let mut edit_name = edit_name.clone();
                                        let mut edit_email = edit_email.clone();
                                        let mut edit_phone = edit_phone.clone();
                                        let mut edit_status = edit_status.clone();
                                        let mut edit_image_name = edit_image_name.clone();
                                        let mut edit_image_type = edit_image_type.clone();
                                        let mut edit_image_payload = edit_image_payload.clone();
                                        let mut edit_image_preview = edit_image_preview.clone();
                                        let mut customers_state = customers_state.clone();
                                        let mut delete_msg = delete_msg.clone();
                                        let mut delete_error = delete_error.clone();
                                        let cid_for_delete = cid.clone();
                                        rsx!(
                                            tr {
                                                td { "{name}" }
                                                td { "{email}" }
                                                td { "{phone}" }
                                                td { span { class: "data-chip", "{status}" } }
                                                td { div { class: "table-actions",
                                                    button {
                                                        class: "btn-small",
                                                        onclick: move |_| {
                                                            edit_id.set(Some(cid.clone()));
                                                            edit_name.set(name.clone());
                                                            edit_email.set(email.clone());
                                                            edit_phone.set(phone.clone());
                                                            edit_status.set(if status.is_empty() { "active".to_string() } else { status.clone() });
                                                            let image_name_opt = if image_name.is_empty() { None } else { Some(image_name.clone()) };
                                                            let image_type_opt = if image_type.is_empty() { None } else { Some(image_type.clone()) };
                                                            edit_image_name.set(image_name_opt.clone());
                                                            edit_image_type.set(image_type_opt.clone());
                                                            edit_image_payload.set(None);
                                                            let preview = image_name_opt
                                                                .as_deref()
                                                                .and_then(|name| image_preview_from_filename(name, &image_type_opt, PROFILES_IMAGES_DIR));
                                                            edit_image_preview.set(preview);
                                                            show_edit_modal.set(true);
                                                        },
                                                        "✏️ Edit"
                                                    }
                                                    button {
                                                        class: "btn-small btn-danger",
                                                        onclick: move |_| {
                                                            let mut updated = customers_state.read().clone();
                                                            updated.retain(|c| pick_first(c, &["id", "customer_id"]) != cid_for_delete);
                                                            match save_extra_to_json("customers.json", &updated) {
                                                                Ok(_) => {
                                                                    customers_state.set(updated);
                                                                    delete_error.set(None);
                                                                    delete_msg.set(Some("✅ Customer deleted.".to_string()));
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

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| customer_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| customer_page.set(current_page + 1),
                    "Next"
                }
            }
            if let Some(msg) = customer_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = customer_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Customer" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }
                        div { class: "form-group",
                            label { "Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    value: "{edit_name}",
                                    oninput: move |e| edit_name.set(e.value()),
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
                        div { class: "form-group",
                            label { "Profile Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut edit_image_payload = edit_image_payload.clone();
                                  let mut edit_image_type = edit_image_type.clone();
                                  let mut edit_image_preview = edit_image_preview.clone();
                                  let mut edit_error = edit_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    edit_image_payload.set(Some(payload));
                                                    edit_image_type.set(Some(img_type));
                                                    edit_image_preview.set(Some(preview));
                                                }
                                                Err(err) => edit_error.set(Some(format!("Image load failed: {}", err))),
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = edit_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_customer,
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
