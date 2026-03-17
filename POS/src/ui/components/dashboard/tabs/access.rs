use super::prelude::*;

#[component]
pub(crate) fn AccessTab() -> Element {
    let store = get_store_fresh();
    let roles = extra_list("roles.json");
    let permissions = extra_list("permissions.json");
    let role_permissions = extra_list("role_permissions.json");
    let user_roles = extra_list("user_roles.json");
    let user_options: Vec<(String, String)> = store
        .users
        .iter()
        .map(|u| (u.id.clone(), format!("{} ({})", u.username, u.id)))
        .collect();
    let role_options: Vec<String> = roles
        .iter()
        .map(|r| pick_first(r, &["name", "title", "role"]))
        .filter(|r| r != "—")
        .collect();
    let perm_options: Vec<String> = permissions
        .iter()
        .map(|p| pick_first(p, &["name", "code", "permission"]))
        .filter(|p| p != "—")
        .collect();
    let mut show_role_modal = use_signal(|| false);
    let mut show_permission_modal = use_signal(|| false);
    let mut show_role_link_modal = use_signal(|| false);
    let mut show_user_role_modal = use_signal(|| false);
    let mut role_name = use_signal(|| String::new());
    let mut role_scope = use_signal(|| String::new());
    let mut perm_name = use_signal(|| String::new());
    let mut perm_group = use_signal(|| String::new());
    let mut link_role = use_signal(|| String::new());
    let mut link_permission = use_signal(|| String::new());
    let mut assign_user = use_signal(|| String::new());
    let mut assign_role = use_signal(|| String::new());
    let mut access_msg = use_signal(|| None::<String>);
    let mut access_error = use_signal(|| None::<String>);
    let mut roles_owned: Vec<serde_json::Value> = roles.iter().map(|r| (*r).clone()).collect();
    let mut permissions_owned: Vec<serde_json::Value> =
        permissions.iter().map(|p| (*p).clone()).collect();
    let mut role_permissions_owned: Vec<serde_json::Value> =
        role_permissions.iter().map(|rp| (*rp).clone()).collect();
    let mut user_roles_owned: Vec<serde_json::Value> =
        user_roles.iter().map(|ur| (*ur).clone()).collect();

    rsx! {
        div { class: "content-card",
            h2 { "🛡️ Access Control" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Roles: {roles.len()}" }
                div { class: "ops-pill", "Permissions: {permissions.len()}" }
                div { class: "ops-pill", "Role links: {role_permissions.len()}" }
                div { class: "ops-pill", "User links: {user_roles.len()}" }
                button {
                    class: "btn-small",
                    onclick: move |_| show_role_modal.set(true),
                    "+ Role"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_permission_modal.set(true),
                    "+ Permission"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_role_link_modal.set(true),
                    "+ Role Link"
                }
                button {
                    class: "btn-small",
                    onclick: move |_| show_user_role_modal.set(true),
                    "+ User Role"
                }
            }

            if let Some(msg) = access_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = access_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Roles" }
                        span { class: "card-note", "{roles.len()} roles" }
                    }
                    div { class: "data-list",
                        if roles.is_empty() {
                            div { class: "empty-state", "No roles configured." }
                        } else {
                            { roles.iter().take(6).map(|r| {
                                let name = pick_first(r, &["name", "title", "role"]);
                                let scope = pick_first(r, &["scope", "level", "description"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{scope}" }
                                        }
                                        span { class: "data-chip", "Role" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Permissions" }
                        span { class: "card-note", "{permissions.len()} permissions" }
                    }
                    div { class: "data-list",
                        if permissions.is_empty() {
                            div { class: "empty-state", "No permissions configured." }
                        } else {
                            { permissions.iter().take(6).map(|p| {
                                let name = pick_first(p, &["name", "code", "permission"]);
                                let group = pick_first(p, &["group", "module", "resource"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{name}" }
                                            span { "{group}" }
                                        }
                                        span { class: "data-chip", "Permission" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Role Assignments" }
                        span { class: "card-note", "{role_permissions.len()} links" }
                    }
                    div { class: "data-list",
                        if role_permissions.is_empty() {
                            div { class: "empty-state", "No role-permission links found." }
                        } else {
                            { role_permissions.iter().take(6).map(|rp| {
                                let role = pick_first(rp, &["role", "role_id", "role_name"]);
                                let perm = pick_first(rp, &["permission", "permission_id", "permission_name"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{role}" }
                                            span { "{perm}" }
                                        }
                                        span { class: "data-chip", "Linked" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "User Roles" }
                        span { class: "card-note", "{user_roles.len()} links" }
                    }
                    div { class: "data-list",
                        if user_roles.is_empty() {
                            div { class: "empty-state", "No user-role assignments found." }
                        } else {
                            { user_roles.iter().take(6).map(|ur| {
                                let user_id = pick_first(ur, &["user_id", "user", "username"]);
                                let role = pick_first(ur, &["role_id", "role", "role_name"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{user_id}" }
                                            span { "{role}" }
                                        }
                                        span { class: "data-chip", "Assigned" }
                                    }
                                )
                            }) }
                        }
                    }
                }
            }

            if show_role_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Add Role" }
                            button { class: "modal-close", onclick: move |_| show_role_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Role Name" }
                            input { value: "{role_name}", oninput: move |e| role_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { "Scope" }
                            input { value: "{role_scope}", oninput: move |e| role_scope.set(e.value()) }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    roles_owned.push(json!({"name": role_name.read().clone(), "scope": role_scope.read().clone()}));
                                    match save_extra_to_json("roles.json", &roles_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Role added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_role_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_role_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_permission_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Add Permission" }
                            button { class: "modal-close", onclick: move |_| show_permission_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Permission Name" }
                            input { value: "{perm_name}", oninput: move |e| perm_name.set(e.value()) }
                        }
                        div { class: "form-group",
                            label { "Group" }
                            input { value: "{perm_group}", oninput: move |e| perm_group.set(e.value()) }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    permissions_owned.push(json!({"name": perm_name.read().clone(), "group": perm_group.read().clone()}));
                                    match save_extra_to_json("permissions.json", &permissions_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Permission added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_permission_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_permission_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_role_link_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Link Role Permission" }
                            button { class: "modal-close", onclick: move |_| show_role_link_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "Role" }
                            if role_options.is_empty() {
                                input { value: "{link_role}", oninput: move |e| link_role.set(e.value()) }
                            } else {
                                select {
                                    value: "{link_role}",
                                    onchange: move |e| link_role.set(e.value()),
                                    option { value: "", "Select role" }
                                    { role_options.iter().map(|r| rsx!( option { value: "{r}", "{r}" } )) }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Permission" }
                            if perm_options.is_empty() {
                                input { value: "{link_permission}", oninput: move |e| link_permission.set(e.value()) }
                            } else {
                                select {
                                    value: "{link_permission}",
                                    onchange: move |e| link_permission.set(e.value()),
                                    option { value: "", "Select permission" }
                                    { perm_options.iter().map(|p| rsx!( option { value: "{p}", "{p}" } )) }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    role_permissions_owned.push(json!({"role": link_role.read().clone(), "permission": link_permission.read().clone()}));
                                    match save_extra_to_json("role_permissions.json", &role_permissions_owned) {
                                        Ok(_) => access_msg.set(Some("✅ Role link added".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_role_link_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_role_link_modal.set(false), "Cancel" }
                        }
                    }
                }
            }

            if show_user_role_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "Assign User Role" }
                            button { class: "modal-close", onclick: move |_| show_user_role_modal.set(false), "✕" }
                        }
                        div { class: "form-group",
                            label { "User ID" }
                            if user_options.is_empty() {
                                input { value: "{assign_user}", oninput: move |e| assign_user.set(e.value()) }
                            } else {
                                select {
                                    value: "{assign_user}",
                                    onchange: move |e| assign_user.set(e.value()),
                                    option { value: "", "Select user" }
                                    { user_options.iter().map(|(id, label)| rsx!( option { value: "{id}", "{label}" } )) }
                                }
                            }
                        }
                        div { class: "form-group",
                            label { "Role" }
                            if role_options.is_empty() {
                                input { value: "{assign_role}", oninput: move |e| assign_role.set(e.value()) }
                            } else {
                                select {
                                    value: "{assign_role}",
                                    onchange: move |e| assign_role.set(e.value()),
                                    option { value: "", "Select role" }
                                    { role_options.iter().map(|r| rsx!( option { value: "{r}", "{r}" } )) }
                                }
                            }
                        }
                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    user_roles_owned.push(json!({"user_id": assign_user.read().clone(), "role": assign_role.read().clone()}));
                                    match save_extra_to_json("user_roles.json", &user_roles_owned) {
                                        Ok(_) => access_msg.set(Some("✅ User role assigned".to_string())),
                                        Err(err) => access_error.set(Some(err)),
                                    }
                                    show_user_role_modal.set(false);
                                },
                                "Save"
                            }
                            button { class: "btn btn-secondary", onclick: move |_| show_user_role_modal.set(false), "Cancel" }
                        }
                    }
                }
            }
        }
    }
}
