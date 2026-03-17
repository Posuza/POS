use super::prelude::*;

#[component]
pub(crate) fn PrintBarcodeTab() -> Element {
    rsx! {
        div { class: "admin-form-page",
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Print Barcode" }
                    span { class: "card-subtitle", "Generate barcode labels for selected products." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Warehouse" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field",
                        label { "Store" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field span-2",
                        label { "Product" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🔍" }
                            input { r#type: "search", placeholder: "Search product by code" }
                        }
                    }
                }
                div { class: "table-container table-compact",
                    table { class: "admin-table",
                        thead {
                            tr {
                                th { "Product" }
                                th { "SKU" }
                                th { "Code" }
                                th { "Qty" }
                            }
                        }
                        tbody {
                            tr {
                                td { colspan: "4",
                                    div { class: "empty-state", "No Data Available" }
                                }
                            }
                        }
                    }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Paper Size" }
                        select { option { "Select" } }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Store Name" }
                        input { r#type: "checkbox", checked: true }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Product Name" }
                        input { r#type: "checkbox", checked: true }
                    }
                    div { class: "form-field toggle-field",
                        label { "Show Price" }
                        input { r#type: "checkbox", checked: true }
                    }
                }
                div { class: "form-actions",
                    button { class: "btn-secondary", "Reset Barcode" }
                    button { class: "btn-primary", "Print Barcode" }
                }
            }
        }
    }
}
