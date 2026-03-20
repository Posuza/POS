use super::prelude::*;

#[component]
pub(crate) fn SuperAdminTab() -> Element {
    let _store = get_store_fresh();
    let mut stores_state = use_signal(|| {
        extra_list("stores.json")
            .iter()
            .map(|v| (*v).clone())
            .collect::<Vec<_>>()
    });
    let mut plans_state = use_signal(|| {
        extra_list("plans.json")
            .iter()
            .map(|v| (*v).clone())
            .collect::<Vec<_>>()
    });
    let mut audits_state = use_signal(|| {
        extra_list("system_audits.json")
            .iter()
            .map(|v| (*v).clone())
            .collect::<Vec<_>>()
    });
    let mut notifications_state = use_signal(|| {
        extra_list("global_notifications.json")
            .iter()
            .map(|v| (*v).clone())
            .collect::<Vec<_>>()
    });

    let mut store_name = use_signal(|| String::new());
    let mut store_owner = use_signal(|| String::new());
    let mut store_email = use_signal(|| String::new());
    let mut store_phone = use_signal(|| String::new());
    let mut store_status = use_signal(|| "Active".to_string());

    let mut plan_name = use_signal(|| String::new());
    let mut plan_price = use_signal(|| String::new());
    let mut plan_cycle = use_signal(|| String::new());

    let mut notice_title = use_signal(|| String::new());
    let mut notice_body = use_signal(|| String::new());

    let mut action_msg = use_signal(|| None::<String>);
    let mut action_error = use_signal(|| None::<String>);

    let handle_create_store = {
        let mut stores_state = stores_state.clone();
        let mut store_name = store_name.clone();
        let mut store_owner = store_owner.clone();
        let mut store_email = store_email.clone();
        let mut store_phone = store_phone.clone();
        let mut store_status = store_status.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if store_name.read().is_empty() {
                action_error.set(Some("Store name is required.".to_string()));
                return;
            }
            let mut updated = stores_state.read().clone();
            let now = now_iso();
            updated.push(json!({
                "id": new_id("st"),
                "store": store_name.read().clone(),
                "user": store_owner.read().clone(),
                "email": store_email.read().clone(),
                "phone": store_phone.read().clone(),
                "status": store_status.read().clone(),
                "created_at": now,
            }));
            match save_extra_to_json("stores.json", &updated) {
                Ok(_) => {
                    stores_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Store created.".to_string()));
                    store_name.set(String::new());
                    store_owner.set(String::new());
                    store_email.set(String::new());
                    store_phone.set(String::new());
                    store_status.set("Active".to_string());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_add_plan = {
        let mut plans_state = plans_state.clone();
        let mut plan_name = plan_name.clone();
        let mut plan_price = plan_price.clone();
        let mut plan_cycle = plan_cycle.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if plan_name.read().is_empty() {
                action_error.set(Some("Plan name is required.".to_string()));
                return;
            }
            let mut updated = plans_state.read().clone();
            updated.push(json!({
                "id": new_id("plan"),
                "name": plan_name.read().clone(),
                "price": plan_price.read().clone(),
                "cycle": plan_cycle.read().clone(),
                "created_at": now_iso(),
            }));
            match save_extra_to_json("plans.json", &updated) {
                Ok(_) => {
                    plans_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Plan added.".to_string()));
                    plan_name.set(String::new());
                    plan_price.set(String::new());
                    plan_cycle.set(String::new());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_run_audit = {
        let mut audits_state = audits_state.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            let mut updated = audits_state.read().clone();
            updated.push(json!({
                "id": new_id("audit"),
                "title": "System audit",
                "status": "completed",
                "created_at": now_iso(),
            }));
            match save_extra_to_json("system_audits.json", &updated) {
                Ok(_) => {
                    audits_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ System audit logged.".to_string()));
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    let handle_add_notification = {
        let mut notifications_state = notifications_state.clone();
        let mut notice_title = notice_title.clone();
        let mut notice_body = notice_body.clone();
        let mut action_msg = action_msg.clone();
        let mut action_error = action_error.clone();
        move |_| {
            if notice_title.read().is_empty() || notice_body.read().is_empty() {
                action_error.set(Some(
                    "Notification title and message are required.".to_string(),
                ));
                return;
            }
            let mut updated = notifications_state.read().clone();
            updated.push(json!({
                "id": new_id("note"),
                "title": notice_title.read().clone(),
                "message": notice_body.read().clone(),
                "created_at": now_iso(),
            }));
            match save_extra_to_json("global_notifications.json", &updated) {
                Ok(_) => {
                    notifications_state.set(updated);
                    action_error.set(None);
                    action_msg.set(Some("✅ Notification queued.".to_string()));
                    notice_title.set(String::new());
                    notice_body.set(String::new());
                }
                Err(err) => action_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "super-admin-page",
            div { class: "sa-topbar",
                div { class: "sa-topbar-left",
                    h2 { "Welcome, Admin" }
                    p { "You have 200+ Orders, Today" }
                }
                div { class: "sa-topbar-right",
                    div { class: "sa-topbar-range",
                        span { "📅" }
                        span { "01 Jan 2024 - 07 Jan 2024" }
                    }
                }
            }

            div { class: "sa-hero-banner",
                div { class: "sa-hero-text",
                    h1 { "Welcome Back, Adrian" }
                    p { "14 New Companies Subscribed Today !!!" }
                }
                div { class: "sa-hero-actions",
                    button { class: "sa-hero-btn btn-dark", "Companies" }
                    button { class: "sa-hero-btn btn-light", "All Packages" }
                }
            }

            div { class: "sa-stat-row",
                div { class: "sa-stat-card",
                    div { class: "sa-stat-header",
                        div { class: "sa-stat-icon slate", "🏢" }
                        span { class: "sa-stat-badge positive", "+19.01%" }
                    }
                    div { class: "sa-stat-body", strong { "5468" } span { "Total Companies" } }
                    div { class: "sa-stat-footer-chart",
                        span { class: "sa-minibar orange" } span { class: "sa-minibar orange" } span { class: "sa-minibar orange" } span { class: "sa-minibar orange" } span { class: "sa-minibar orange" }
                    }
                }
                div { class: "sa-stat-card",
                    div { class: "sa-stat-header",
                        div { class: "sa-stat-icon indigo", "🏷️" }
                        span { class: "sa-stat-badge positive", "-12%" }
                    }
                    div { class: "sa-stat-body", strong { "4598" } span { "Active Companies" } }
                    div { class: "sa-stat-footer-chart",
                        span { class: "sa-minibar indigo" } span { class: "sa-minibar indigo" } span { class: "sa-minibar indigo" } span { class: "sa-minibar indigo" } span { class: "sa-minibar indigo" }
                    }
                }
                div { class: "sa-stat-card",
                    div { class: "sa-stat-header",
                        div { class: "sa-stat-icon slate", "👥" }
                        span { class: "sa-stat-badge positive", "+6%" }
                    }
                    div { class: "sa-stat-body", strong { "3698" } span { "Total Subscribers" } }
                    div { class: "sa-stat-footer-chart",
                        span { class: "sa-minibar blue" } span { class: "sa-minibar blue" } span { class: "sa-minibar blue" } span { class: "sa-minibar blue" } span { class: "sa-minibar blue" }
                    }
                }
                div { class: "sa-stat-card",
                    div { class: "sa-stat-header",
                        div { class: "sa-stat-icon slate", "💰" }
                        span { class: "sa-stat-badge negative", "-16%" }
                    }
                    div { class: "sa-stat-body", strong { "$89,878.58" } span { "Total Earnings" } }
                    div { class: "sa-stat-footer-chart",
                        span { class: "sa-minibar green" } span { class: "sa-minibar green" } span { class: "sa-minibar green" } span { class: "sa-minibar green" } span { class: "sa-minibar green" }
                    }
                }
            }

            div { class: "sa-grid-3",
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Companies" } select { class: "sa-card-filter", option { "This Month" } option { "This Year" } } }
                    div { class: "sa-revenue-chart placeholder-chart" }
                }
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Revenue" } select { class: "sa-card-filter", option { "This Month" } option { "This Year" } } }
                    div { class: "sa-revenue-chart placeholder-chart" }
                }
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Top Plans" } select { class: "sa-card-filter", option { "This Month" } option { "This Year" } } }
                    div { class: "sa-top-plans-donut",
                        div { class: "sa-donut-wrap",
                            div { class: "sa-donut" }
                            div { class: "sa-donut-inner", "7,185" }
                        }
                        div { class: "sa-plan-legend",
                            div { class: "sa-plan-leg-item",
                                div { class: "sa-plan-leg-label", span { class: "sa-plan-dot orange" } span { "Growth" } }
                                span { class: "sa-plan-pct", "35%" }
                            }
                            div { class: "sa-plan-leg-item",
                                div { class: "sa-plan-leg-label", span { class: "sa-plan-dot yellow" } span { "Pro" } }
                                span { class: "sa-plan-pct", "45%" }
                            }
                            div { class: "sa-plan-leg-item",
                                div { class: "sa-plan-leg-label", span { class: "sa-plan-dot blue" } span { "Enterprise" } }
                                span { class: "sa-plan-pct", "20%" }
                            }
                        }
                    }
                }
            }

            div { class: "sa-grid-3",
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Recent Transactions" } button { class: "sa-action-link", "View All" } }
                    div { class: "sa-list",
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "A" } div { class: "sa-list-info", strong { "AdrianTech" } span { "Sep 15, 2024" } } }
                            div { class: "sa-list-action", strong { "$500" } span { "Pro Plan" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "B" } div { class: "sa-list-info", strong { "ByteCorp" } span { "Sep 14, 2024" } } }
                            div { class: "sa-list-action", strong { "$250" } span { "Growth Plan" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "Z" } div { class: "sa-list-info", strong { "Zenith Inc." } span { "Sep 14, 2024" } } }
                            div { class: "sa-list-action", strong { "$1000" } span { "Enterprise" } }
                        }
                    }
                }
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Recently Registered" } button { class: "sa-action-link", "View All" } }
                    div { class: "sa-list",
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "G" } div { class: "sa-list-info", strong { "Global Retail" } span { "London, UK" } } }
                            div { class: "sa-list-action", strong { "Active" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "S" } div { class: "sa-list-info", strong { "Swift Mart" } span { "Toronto, UK" } } }
                            div { class: "sa-list-action", strong { "Active" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "V" } div { class: "sa-list-info", strong { "Value Store" } span { "New York, USA" } } }
                            div { class: "sa-list-action", strong { "Pending" } }
                        }
                    }
                }
                div { class: "sa-content-card",
                    div { class: "sa-card-header", h3 { "Recent Plan Expired" } button { class: "sa-action-link", "View All" } }
                    div { class: "sa-list",
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "N" } div { class: "sa-list-info", strong { "Nova Shop" } span { "Nov 01, 2024" } } }
                            div { class: "sa-list-action", strong { color: "#dc2626", "Expired" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "O" } div { class: "sa-list-info", strong { "Omega Grocers" } span { "Oct 28, 2024" } } }
                            div { class: "sa-list-action", strong { color: "#dc2626", "Expired" } }
                        }
                        div { class: "sa-list-row",
                            div { class: "sa-list-main", span { class: "sa-list-avatar", "P" } div { class: "sa-list-info", strong { "Prime Mart" } span { "Oct 15, 2024" } } }
                            div { class: "sa-list-action", strong { color: "#dc2626", "Expired" } }
                        }
                    }
                }
            }

            if let Some(msg) = action_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = action_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Create Store" }
                    span { class: "card-subtitle", "Provision a new store and owner details." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Store Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏪" }
                            input {
                                placeholder: "Downtown Outlet",
                                value: "{store_name}",
                                oninput: move |e| store_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Owner / Manager" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "👤" }
                            input {
                                placeholder: "manager01",
                                value: "{store_owner}",
                                oninput: move |e| store_owner.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Email" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "✉️" }
                            input {
                                placeholder: "store@example.com",
                                value: "{store_email}",
                                oninput: move |e| store_email.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Phone" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📞" }
                            input {
                                placeholder: "+1-555-0100",
                                value: "{store_phone}",
                                oninput: move |e| store_phone.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Status" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{store_status}",
                                onchange: move |e| store_status.set(e.value()),
                                option { value: "Active", "Active" }
                                option { value: "Inactive", "Inactive" }
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-primary",
                        onclick: handle_create_store,
                        "Create Store"
                    }
                }
                div { class: "data-list",
                    { stores_state.read().iter().take(6).map(|s| {
                        let name = pick_first(s, &["store", "name"]);
                        let owner = pick_first(s, &["user", "owner"]);
                        let status = pick_first(s, &["status"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{name}" }
                                    span { "{owner}" }
                                }
                                span { class: "data-chip", "{status}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Manage Plans" }
                    span { class: "card-subtitle", "Add or adjust subscription plans." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Plan Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏷️" }
                            input {
                                placeholder: "Growth",
                                value: "{plan_name}",
                                oninput: move |e| plan_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Price" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "💲" }
                            input {
                                placeholder: "$49",
                                value: "{plan_price}",
                                oninput: move |e| plan_price.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Billing Cycle" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🗓️" }
                            input {
                                placeholder: "Monthly",
                                value: "{plan_cycle}",
                                oninput: move |e| plan_cycle.set(e.value()),
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-secondary",
                        onclick: handle_add_plan,
                        "Add Plan"
                    }
                }
                div { class: "data-list",
                    { plans_state.read().iter().take(6).map(|p| {
                        let name = pick_first(p, &["name", "title"]);
                        let price = pick_first(p, &["price", "amount"]);
                        let cycle = pick_first(p, &["cycle", "billing"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{name}" }
                                    span { "{cycle}" }
                                }
                                span { class: "data-chip", "{price}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "System Audit" }
                    span { class: "card-subtitle", "Log a system audit run." }
                }
                div { class: "admin-card-body grid-actions",
                    button { class: "btn-secondary", onclick: handle_run_audit, "Run System Audit" }
                }
                div { class: "data-list",
                    { audits_state.read().iter().rev().take(5).map(|a| {
                        let title = pick_first(a, &["title", "name"]);
                        let status = pick_first(a, &["status"]);
                        let date = pick_first(a, &["created_at", "created"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{title}" }
                                    span { "{date}" }
                                }
                                span { class: "data-chip", "{status}" }
                            }
                        )
                    }) }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Global Notifications" }
                    span { class: "card-subtitle", "Push announcements to all stores." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field span-2",
                        label { "Title" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📣" }
                            input {
                                placeholder: "Holiday Hours Update",
                                value: "{notice_title}",
                                oninput: move |e| notice_title.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Message" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            textarea {
                                rows: "3",
                                placeholder: "All stores close at 6 PM this Friday.",
                                value: "{notice_body}",
                                oninput: move |e| notice_body.set(e.value()),
                            }
                        }
                    }
                }
                div { class: "form-actions",
                    button {
                        class: "btn-secondary",
                        onclick: handle_add_notification,
                        "Send Notification"
                    }
                }
                div { class: "data-list",
                    { notifications_state.read().iter().rev().take(5).map(|n| {
                        let title = pick_first(n, &["title", "name"]);
                        let message = pick_first(n, &["message", "body"]);
                        rsx!(
                            div { class: "data-row",
                                div { class: "data-main",
                                    strong { "{title}" }
                                    span { "{message}" }
                                }
                            }
                        )
                    }) }
                }
            }
        }
    }
}
