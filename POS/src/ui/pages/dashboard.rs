use crate::config::constants::APP_NAME;
use crate::data::json_store::get_store_fresh;
use crate::data::models::user::User;
use crate::ui::components::dashboard::sidebar_menu::{AdminTab, SidebarMenu};
use crate::ui::components::dashboard::tabs::{
    AccessTab, BillersTab, BrandsTab, CategoriesTab, CreateProductTab, CustomerReportTab,
    CustomersTab, DashboardTab, ExpiredProductsTab, InventoryReportTab, InventoryTab,
    InvoiceReportTab, LowStocksTab, ManageStockTab, PaymentsTab, PlaceholderTab, PrintBarcodeTab,
    PrintQrCodeTab, ProductReportTab, ProductsTab, PurchaseReportTab, SalesDashboardTab,
    SalesReportTab, SalesTab, SettingsTab, ShiftsTab, StaffTab, StockAdjustmentTab,
    StockTransferTab, StoresTab, SubCategoriesTab, SuperAdminTab, SupplierReportTab, SuppliersTab,
    UnitsTab, VariantAttributesTab, WarehousesTab, WarrantiesTab,
};
use crate::ui::components::Navbar;
use crate::utils::formatters::format_datetime;
use dioxus::prelude::*;

#[component]
pub fn AdminPage(user: User, on_logout: EventHandler<()>) -> Element {
    let mut active_tab = use_signal(|| AdminTab::Dashboard);
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        Navbar {
            title: APP_NAME.to_string(),
            user: Some(user.clone()),
            on_login: move |_| (),
            on_logout: on_logout.clone(),
        }

        SidebarMenu {
            active_tab: active_tab.clone(),
            on_select: move |tab: AdminTab| active_tab.set(tab),
            is_open: sidebar_open.clone(),
            on_close: move |_| sidebar_open.set(false),
        }

        div { class: "admin-page",
            div { class: "admin-container",
                div {
                    class: "admin-menu-toggle-wrap",
                    button {
                        class: "admin-menu-toggle",
                        aria_label: "Open admin menu",
                        onclick: move |_| {
                            let next_state = !*sidebar_open.read();
                            sidebar_open.set(next_state);
                        },
                        span { class: "admin-menu-toggle-icon", "☰" }
                        span { class: "admin-menu-toggle-label", "Menu" }
                    }
                }

                div { class: "admin-main",
                    {
                        let (tab_label, tab_subtitle) = tab_metadata(*active_tab.read());
                        let store = get_store_fresh();
                        let last_updated = store.sales
                            .last()
                            .map(|s| format_datetime(&s.created_at))
                            .unwrap_or_else(|| "No recent updates".to_string());
                        rsx!(
                            div { class: "admin-page-header",
                                div { class: "breadcrumb",
                                    span { "Admin" }
                                    span { class: "breadcrumb-sep", "›" }
                                    span { class: "breadcrumb-current", "{tab_label}" }
                                }
                                h1 { class: "page-title", "{tab_label}" }
                                p { class: "page-subtitle", "{tab_subtitle}" }
                                span { class: "page-updated", "Last updated: {last_updated}" }
                            }
                        )
                    }
                    match *active_tab.read() {
                        AdminTab::Dashboard => rsx! { DashboardTab { user: user.clone() } },
                        AdminTab::SuperAdmin => rsx! { SuperAdminTab {} },
                        AdminTab::SalesDashboard => rsx! { SalesDashboardTab {} },
                        AdminTab::Sales => rsx! { SalesTab {} },
                        AdminTab::Products => rsx! { ProductsTab {} },
                        AdminTab::CreateProduct => rsx! { CreateProductTab {} },
                        AdminTab::ExpiredProducts => rsx! { ExpiredProductsTab {} },
                        AdminTab::LowStocks => rsx! { LowStocksTab {} },
                        AdminTab::Categories => rsx! { CategoriesTab {} },
                        AdminTab::SubCategories => rsx! { SubCategoriesTab {} },
                        AdminTab::Brands => rsx! { BrandsTab {} },
                        AdminTab::Units => rsx! { UnitsTab {} },
                        AdminTab::VariantAttributes => rsx! { VariantAttributesTab {} },
                        AdminTab::Warranties => rsx! { WarrantiesTab {} },
                        AdminTab::PrintBarcode => rsx! { PrintBarcodeTab {} },
                        AdminTab::PrintQrCode => rsx! { PrintQrCodeTab {} },
                        AdminTab::Warehouses => rsx! { WarehousesTab {} },
                        AdminTab::Stores => rsx! { StoresTab {} },
                        AdminTab::Billers => rsx! { BillersTab {} },
                        AdminTab::ManageStock => rsx! { ManageStockTab {} },
                        AdminTab::StockAdjustment => rsx! { StockAdjustmentTab {} },
                        AdminTab::StockTransfer => rsx! { StockTransferTab {} },
                        AdminTab::InvoiceReport => rsx! { InvoiceReportTab {} },
                        AdminTab::SupplierReport => rsx! { SupplierReportTab {} },
                        AdminTab::CustomerReport => rsx! { CustomerReportTab {} },
                        AdminTab::ProductReport => rsx! { ProductReportTab {} },
                        AdminTab::InventoryReport => rsx! { InventoryReportTab {} },
                        AdminTab::PurchaseReport => rsx! { PurchaseReportTab {} },
                        AdminTab::SalesReport => rsx! { SalesReportTab {} },
                        AdminTab::Staff => rsx! { StaffTab {} },
                        AdminTab::Customers => rsx! { CustomersTab {} },
                        AdminTab::Inventory => rsx! { InventoryTab {} },
                        AdminTab::Payments => rsx! { PaymentsTab {} },
                        AdminTab::Access => rsx! { AccessTab {} },
                        AdminTab::Suppliers => rsx! { SuppliersTab {} },
                        AdminTab::Shifts => rsx! { ShiftsTab {} },
                        AdminTab::Settings => rsx! { SettingsTab {} },
                        AdminTab::Placeholder(title) => rsx! { PlaceholderTab { title: title } },
                    }
                }
            }
        }
    }
}

fn tab_metadata(tab: AdminTab) -> (&'static str, &'static str) {
    match tab {
        AdminTab::Dashboard => ("Dashboard", "Live operational overview and insights."),
        AdminTab::SuperAdmin => ("Super Admin", "System-wide access and master controls."),
        AdminTab::SalesDashboard => (
            "Sales Dashboard",
            "Sales KPIs, trends, and recent performance.",
        ),
        AdminTab::Sales => ("Sales", "Review transactions and revenue performance."),
        AdminTab::SalesReport => ("Sales Report", "Summary of sales and performance trends."),
        AdminTab::Products => ("Products", "Manage inventory and product catalog."),
        AdminTab::CreateProduct => ("Create Product", "Add a new item to your catalog."),
        AdminTab::ExpiredProducts => (
            "Expired Products",
            "Track expired or near-expiry inventory.",
        ),
        AdminTab::LowStocks => ("Low Stocks", "Monitor products below reorder levels."),
        AdminTab::Categories => ("Category", "Organize product categories."),
        AdminTab::SubCategories => ("Sub Category", "Organize product subcategories."),
        AdminTab::Brands => ("Brands", "Manage product brands."),
        AdminTab::Units => ("Units", "Configure measurement units."),
        AdminTab::VariantAttributes => {
            ("Variant Attributes", "Define product variants and options.")
        }
        AdminTab::Warranties => ("Warranties", "Manage warranty coverage details."),
        AdminTab::PrintBarcode => ("Print Barcode", "Generate and print product barcodes."),
        AdminTab::PrintQrCode => ("Print QR Code", "Generate and print product QR codes."),
        AdminTab::Warehouses => ("Warehouses", "Manage warehouse locations and stock."),
        AdminTab::Stores => ("Stores", "Manage store locations and contacts."),
        AdminTab::Billers => ("Billers", "Manage billing contacts and partners."),
        AdminTab::ManageStock => ("Manage Stock", "Track stock adjustments and assignments."),
        AdminTab::StockAdjustment => ("Stock Adjustment", "Record stock corrections and audits."),
        AdminTab::StockTransfer => ("Stock Transfer", "Move inventory between locations."),
        AdminTab::InvoiceReport => ("Invoice Report", "Invoice status and collections overview."),
        AdminTab::SupplierReport => ("Supplier Report", "Supplier activity and balances."),
        AdminTab::CustomerReport => ("Customer Report", "Customer orders and payment behavior."),
        AdminTab::ProductReport => ("Product Report", "Top products and inventory revenue."),
        AdminTab::InventoryReport => ("Inventory Report", "Inventory summary and stock snapshots."),
        AdminTab::PurchaseReport => ("Purchase Report", "Purchases and supplier spend details."),
        AdminTab::Staff => ("Staff", "Control staff access and schedules."),
        AdminTab::Customers => ("Customers", "Track customer profiles and engagement."),
        AdminTab::Inventory => ("Inventory", "Monitor stock movements and scan activity."),
        AdminTab::Payments => ("Payments", "Review payment records and settlement status."),
        AdminTab::Access => (
            "Access Control",
            "Manage roles, permissions, and assignments.",
        ),
        AdminTab::Suppliers => ("Suppliers", "Coordinate supplier contacts and performance."),
        AdminTab::Shifts => ("Shifts", "Coordinate staffing coverage and schedules."),
        AdminTab::Settings => (
            "Settings",
            "Configure store preferences and system settings.",
        ),
        AdminTab::Placeholder(label) => (label, "Coming soon."),
    }
}
