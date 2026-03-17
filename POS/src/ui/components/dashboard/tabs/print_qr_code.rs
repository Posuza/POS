use super::prelude::*;

#[component]
pub(crate) fn PrintQrCodeTab() -> Element {
    rsx! {
        div { class: "admin-form-page",
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Print QR Code" }
                    span { class: "card-subtitle", "Create QR codes for inventory labels." }
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
                                th { "Reference Number" }
                                th { "Qty" }
                            }
                        }
                        tbody {
                            tr {
                                td { colspan: "5",
                                    div { class: "empty-state", "No Data Available" }
                                }
                            }
                        }
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
