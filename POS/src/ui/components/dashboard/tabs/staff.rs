use super::prelude::*;

#[component]
pub(crate) fn StaffTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut staff_query = use_signal(|| String::new());
    let mut staff_sort = use_signal(|| "name".to_string());
    let mut staff_order = use_signal(|| "asc".to_string());
    let mut staff_page = use_signal(|| 1usize);
    let mut staff_export_msg = use_signal(|| None::<String>);
    let mut staff_export_error = use_signal(|| None::<String>);
    let mut staff_username = use_signal(|| String::new());
    let mut staff_email = use_signal(|| String::new());
    let mut staff_password = use_signal(|| String::new());
    let mut staff_image = use_signal(|| None::<String>);
    let store = get_store_fresh();
    let users = store.users.clone();
    let sales = &store.sales;
    
    let handle_add_staff = move |_| {
        if staff_username.read().is_empty() || staff_email.read().is_empty() {
            return;
        }
        // In real app, save to database with image in data/images/profiles/
        staff_username.set(String::new());
        staff_email.set(String::new());
        staff_password.set(String::new());
        staff_image.set(None);
        show_add_modal.set(false);
    };
    
    let query = staff_query.read().to_lowercase();
    let mut filtered_staff: Vec<crate::data::json_store::UserRecord> = users
        .clone()
        .into_iter()
        .filter(|u| {
            if query.is_empty() {
                true
            } else {
                let name = u.username.to_lowercase();
                let email = u.email.to_lowercase();
                let role = format!("{:?}", u.role).to_lowercase();
                let status = u.status.to_lowercase();
                name.contains(&query) || email.contains(&query) || role.contains(&query) || status.contains(&query)
            }
        })
        .collect();
    let sort_key = staff_sort.read().clone();
    filtered_staff.sort_by(|a, b| {
        match sort_key.as_str() {
            "email" => a.email.to_lowercase().cmp(&b.email.to_lowercase()),
            "status" => a.status.to_lowercase().cmp(&b.status.to_lowercase()),
            _ => a.username.to_lowercase().cmp(&b.username.to_lowercase()),
        }
    });
    if staff_order.read().as_str() == "desc" {
        filtered_staff.reverse();
    }
    let page_size = 10usize;
    let total_pages = std::cmp::max(1, (filtered_staff.len() + page_size - 1) / page_size);
    let current_page = (*staff_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_staff.len());
    let page_items: Vec<crate::data::json_store::UserRecord> = if filtered_staff.is_empty() {
        Vec::new()
    } else {
        filtered_staff[start..end].to_vec()
    };
    let mut staff_rows: Vec<Element> = Vec::new();
    for u in page_items.into_iter() {
        let status_label = if u.status == "active" { "✅ Active" } else { "⏸️ Inactive" };
        let role_label = match u.role {
            crate::data::models::user::UserRole::Admin => "🛡️ Admin",
            crate::data::models::user::UserRole::Staff => "👤 Staff",
        };
        let initials = {
            let mut chars = u.username.chars().filter(|c| c.is_alphabetic());
            let first = chars.next().unwrap_or('U');
            let second = chars.next().unwrap_or(first);
            format!("{}{}", first, second).to_uppercase()
        };
        let uname = u.username.clone();
        let email = u.email.clone();
        let img = u.profile_image.clone();
        staff_rows.push(rsx!(
            tr {
                td {
                    if let Some(img) = img {
                        img { class: "table-avatar", src: "{img}" }
                    } else {
                        span { class: "table-avatar-fallback", "{initials}" }
                    }
                }
                td { "{uname}" }
                td { "{email}" }
                td { "{role_label}" }
                td { "{status_label}" }
                td {
                    div { class: "table-actions",
                        button { class: "btn-small", "✏️ Edit" }
                        button { class: "btn-small btn-danger", "🗑️ Remove" }
                    }
                }
            }
        ));
    }

    let mut staff_perf: HashMap<String, (i32, f32)> = HashMap::new();
    for s in sales.iter() {
        let entry = staff_perf.entry(s.cashier_id.clone()).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += s.total;
    }
    let mut staff_perf_vec: Vec<(String, i32, f32)> = staff_perf
        .into_iter()
        .map(|(id, (count, total))| (id, count, total))
        .collect();
    staff_perf_vec.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    rsx! {
        TabContainer {
            div { class: "products-header",
                h2 { "👥 Staff Management" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| show_add_modal.set(true),
                    "+ Add Staff"
                }
            }

            div { class: "ops-summary",
                div { class: "filter-group filter-inline",
                    label { "Search" }
                    input {
                        placeholder: "Name, email, role, status",
                        value: "{staff_query}",
                        oninput: move |e| {
                            staff_query.set(e.value());
                            staff_page.set(1);
                        },
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Sort" }
                    select {
                        value: "{staff_sort}",
                        onchange: move |e| staff_sort.set(e.value()),
                        option { value: "name", "Name" }
                        option { value: "email", "Email" }
                        option { value: "status", "Status" }
                    }
                }
                div { class: "filter-group filter-inline",
                    label { "Order" }
                    select {
                        value: "{staff_order}",
                        onchange: move |e| staff_order.set(e.value()),
                        option { value: "asc", "Asc" }
                        option { value: "desc", "Desc" }
                    }
                }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_staff_csv(&users) {
                            Ok(path) => {
                                staff_export_error.set(None);
                                staff_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => staff_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Avatar" }
                            th { "Username" }
                            th { "Email" }
                            th { "Role" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if staff_rows.is_empty() {
                            tr {
                                td { colspan: "6",
                                    div { class: "empty-state", "No staff users found." }
                                }
                            }
                        } else {
                            { staff_rows.into_iter() }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| staff_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| staff_page.set(current_page + 1),
                    "Next"
                }
            }

            if let Some(msg) = staff_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = staff_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Top Staff Performance" }
                        span { class: "card-note", "By sales volume" }
                    }
                    div { class: "data-list",
                        if staff_perf_vec.is_empty() {
                            div { class: "empty-state", "No sales data yet." }
                        } else {
                            { staff_perf_vec.iter().take(4).map(|(id, count, total)| {
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{id}" }
                                            span { "{count} sales" }
                                        }
                                        span { class: "data-chip", "{format_price(*total)}" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            if show_add_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "➕ Add New Staff" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_add_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Username *" }
                            input {
                                placeholder: "e.g., john",
                                value: "{staff_username}",
                                oninput: move |e| staff_username.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Email *" }
                            input {
                                r#type: "email",
                                placeholder: "e.g., john@pos.local",
                                value: "{staff_email}",
                                oninput: move |e| staff_email.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Password *" }
                            input {
                                r#type: "password",
                                placeholder: "••••••••",
                                value: "{staff_password}",
                                oninput: move |e| staff_password.set(e.value()),
                            }
                        }

                        div { class: "form-group",
                            label { "Profile Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                input {
                                    r#type: "file",
                                    accept: "image/*",
                                    onchange: move |_e| {
                                        // In real app, handle file upload to data/images/profiles/
                                    },
                                }
                            }
                            if let Some(img) = staff_image.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_add_staff,
                                "✅ Add Staff"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_add_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }
        }
    }
}
