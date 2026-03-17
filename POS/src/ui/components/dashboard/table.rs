/// Table component
use dioxus::prelude::*;

/// Data table component
#[component]
pub fn Table() -> Element {
    rsx! {
        div { class: "table-responsive table-container",
            table {
                thead {
                    tr { th { "Column" } }
                }
                tbody {
                    tr { td { "No rows" } }
                }
            }
        }
    }
}
