use super::prelude::*;

#[component]
pub(crate) fn ShiftsTab() -> Element {
    let store = get_store_fresh();
    let shifts = extra_list("shifts.json");
    let open_shifts = shifts
        .iter()
        .filter(|s| value_is(s, "status", "open"))
        .count();
    let closed_shifts = shifts
        .iter()
        .filter(|s| value_is(s, "status", "closed"))
        .count();

    rsx! {
        div { class: "content-card",
            h2 { "🕒 Shifts" }
            div { class: "ops-summary",
                div { class: "ops-pill", "Total: {shifts.len()}" }
                div { class: "ops-pill success", "Open: {open_shifts}" }
                div { class: "ops-pill warning", "Closed: {closed_shifts}" }
            }

            div { class: "ops-grid",
                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Recent Shifts" }
                        span { class: "card-note", "{shifts.len()} shifts" }
                    }
                    div { class: "data-list",
                        if shifts.is_empty() {
                            div { class: "empty-state", "No shifts available." }
                        } else {
                            { shifts.iter().rev().take(6).map(|s| {
                                let staff = pick_first(s, &["staff_name", "staff_id", "user_id"]);
                                let time = pick_first(s, &["start_time", "start", "created_at"]);
                                let status = pick_first(s, &["status", "state"]);
                                rsx!(
                                    div { class: "data-row",
                                        div { class: "data-main",
                                            strong { "{staff}" }
                                            span { "{time}" }
                                        }
                                        span { class: "data-chip", "{status}" }
                                    }
                                )
                            }) }
                        }
                    }
                }

                div { class: "ops-card",
                    div { class: "card-header-row",
                        h3 { "Coverage Notes" }
                        span { class: "card-note", "Staffing" }
                    }
                    div { class: "empty-state", "Add coverage gaps, handoff notes, and approvals here." }
                }
            }
        }
    }
}
