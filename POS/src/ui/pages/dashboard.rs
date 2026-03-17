use dioxus::prelude::*;
use crate::data::models::user::User;
use crate::data::json_store::get_store_fresh;
use crate::utils::formatters::format_datetime;
use crate::ui::components::Navbar;
use crate::ui::components::dashboard::tabs::*;
use crate::config::constants::APP_NAME;

#[component]
pub fn AdminPage(user: User, on_logout: EventHandler<()>) -> Element {
    let mut active_tab = use_signal(|| AdminTab::Dashboard);
    let mut sidebar_open = use_signal(|| false);

    rsx! {
        div {
            Navbar { 
                title: APP_NAME.to_string(),
                user: Some(user.clone()),
                on_login: move |_| (),
                on_logout: on_logout.clone(),
            }

            // Floating Sidebar Menu
            SidebarMenu {
                active_tab: active_tab.clone(),
                on_select: move |tab: AdminTab| active_tab.set(tab),
                is_open: sidebar_open.clone(),
                on_close: move |_| sidebar_open.set(false),
            }

            div { class: "admin-page",
                div { class: "admin-container",
                    // Menu toggle button (visible on all screens, especially mobile)
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

                    // Content
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum AdminTab {
    Dashboard,
    SuperAdmin,
    SalesDashboard,
    Sales,
    Products,
    CreateProduct,
    ExpiredProducts,
    LowStocks,
    Categories,
    SubCategories,
    Brands,
    Units,
    VariantAttributes,
    Warranties,
    PrintBarcode,
    PrintQrCode,
    Warehouses,
    Stores,
    Billers,
    ManageStock,
    StockAdjustment,
    StockTransfer,
    InvoiceReport,
    SupplierReport,
    CustomerReport,
    ProductReport,
    InventoryReport,
    PurchaseReport,
    SalesReport,
    Staff,
    Customers,
    Inventory,
    Payments,
    Access,
    Suppliers,
    Shifts,
    Settings,
    Placeholder(&'static str),
}

#[component]
fn SidebarMenu(active_tab: Signal<AdminTab>, on_select: EventHandler<AdminTab>, is_open: Signal<bool>, on_close: EventHandler<()>) -> Element {
    let drawer_open = *is_open.read();
    let current_tab = *active_tab.read();
    let menu_groups: Vec<(&'static str, Vec<(AdminTab, &'static str, &'static str)>)> = vec![
        ("Main", vec![
            (AdminTab::Dashboard, "Dashboard", "🏠"),
            (AdminTab::SuperAdmin, "Super Admin", "🧭"),
        ]),
        ("Inventory", vec![
            (AdminTab::Products, "Products", "📦"),
            (AdminTab::CreateProduct, "Create Product", "➕"),
            (AdminTab::ExpiredProducts, "Expired Products", "⏳"),
            (AdminTab::LowStocks, "Low Stocks", "⚠️"),
            (AdminTab::Categories, "Category", "🗂️"),
            (AdminTab::SubCategories, "Sub Category", "🧩"),
            (AdminTab::Brands, "Brands", "🏷️"),
            (AdminTab::Units, "Units", "📏"),
            (AdminTab::VariantAttributes, "Variant Attributes", "🧬"),
            (AdminTab::Warranties, "Warranties", "🛡️"),
            (AdminTab::PrintBarcode, "Print Barcode", "🏷️"),
            (AdminTab::PrintQrCode, "Print QR Code", "🔳"),
        ]),
        ("Stock", vec![
            (AdminTab::ManageStock, "Manage Stock", "📦"),
            (AdminTab::StockAdjustment, "Stock Adjustment", "🛠️"),
            (AdminTab::StockTransfer, "Stock Transfer", "🔁"),
        ]),
        ("Sales", vec![
            (AdminTab::SalesDashboard, "Sales Dashboard", "📊"),
            (AdminTab::Sales, "Sales", "💸"),
            (AdminTab::Placeholder("Invoices"), "Invoices", "🧾"),
            (AdminTab::Placeholder("Sales Return"), "Sales Return", "↩️"),
            (AdminTab::Placeholder("Quotation"), "Quotation", "📝"),
            (AdminTab::Placeholder("POS"), "POS", "🛒"),
        ]),
        ("Promo", vec![
            (AdminTab::Placeholder("Coupons"), "Coupons", "🏷️"),
            (AdminTab::Placeholder("Gift Card"), "Gift Card", "🎁"),
            (AdminTab::Placeholder("Discount"), "Discount", "💯"),
        ]),
        ("Purchases", vec![
            (AdminTab::Placeholder("Purchases"), "Purchases", "🧺"),
            (AdminTab::Placeholder("Purchase Order"), "Purchase Order", "📋"),
            (AdminTab::Placeholder("Purchase Return"), "Purchase Return", "↩️"),
        ]),
        ("Finance & Accounts", vec![
            (AdminTab::Placeholder("Expenses"), "Expenses", "💼"),
            (AdminTab::Placeholder("Income"), "Income", "💰"),
            (AdminTab::Placeholder("Bank Accounts"), "Bank Accounts", "🏦"),
            (AdminTab::Placeholder("Money Transfer"), "Money Transfer", "💸"),
            (AdminTab::Placeholder("Balance Sheet"), "Balance Sheet", "📄"),
            (AdminTab::Placeholder("Trial Balance"), "Trial Balance", "🧮"),
            (AdminTab::Placeholder("Cash Flow"), "Cash Flow", "🌊"),
            (AdminTab::Placeholder("Account Statement"), "Account Statement", "📃"),
        ]),
        ("People", vec![
            (AdminTab::Customers, "Customers", "🧑‍🤝‍🧑"),
            (AdminTab::Billers, "Billers", "🧾"),
            (AdminTab::Suppliers, "Suppliers", "🚚"),
            (AdminTab::Stores, "Stores", "🏪"),
            (AdminTab::Warehouses, "Warehouses", "🏭"),
        ]),
        ("HRM", vec![
            (AdminTab::Placeholder("Employees"), "Employees", "👥"),
            (AdminTab::Placeholder("Departments"), "Departments", "🏢"),
            (AdminTab::Placeholder("Designation"), "Designation", "🎯"),
            (AdminTab::Shifts, "Shifts", "🕒"),
            (AdminTab::Placeholder("Attendance"), "Attendance", "📌"),
            (AdminTab::Placeholder("Leaves"), "Leaves", "🌴"),
            (AdminTab::Placeholder("Holidays"), "Holidays", "🎉"),
            (AdminTab::Placeholder("Payroll"), "Payroll", "💵"),
        ]),
        ("Reports", vec![
            (AdminTab::SalesReport, "Sales Report", "📈"),
            (AdminTab::PurchaseReport, "Purchase Report", "🧾"),
            (AdminTab::InventoryReport, "Inventory Report", "📦"),
            (AdminTab::InvoiceReport, "Invoice Report", "🧾"),
            (AdminTab::SupplierReport, "Supplier Report", "🚚"),
            (AdminTab::CustomerReport, "Customer Report", "🧑‍🤝‍🧑"),
            (AdminTab::ProductReport, "Product Report", "📊"),
            (AdminTab::Placeholder("Expense Report"), "Expense Report", "💼"),
            (AdminTab::Placeholder("Income Report"), "Income Report", "💰"),
            (AdminTab::Placeholder("Tax Report"), "Tax Report", "🧾"),
            (AdminTab::Placeholder("Profit & Loss"), "Profit & Loss", "📈"),
            (AdminTab::Placeholder("Annual Report"), "Annual Report", "📅"),
        ]),
        ("User Management", vec![
            (AdminTab::Placeholder("Users"), "Users", "👤"),
            (AdminTab::Access, "Roles & Permissions", "🛡️"),
            (AdminTab::Placeholder("Delete Account Request"), "Delete Account Request", "🗑️"),
        ]),
        ("Content (CMS)", vec![
            (AdminTab::Placeholder("Pages"), "Pages", "📄"),
            (AdminTab::Placeholder("Blog"), "Blog", "📰"),
            (AdminTab::Placeholder("Location"), "Location", "📍"),
            (AdminTab::Placeholder("Testimonials"), "Testimonials", "💬"),
            (AdminTab::Placeholder("FAQ"), "FAQ", "❓"),
        ]),
        ("Pages", vec![
            (AdminTab::Placeholder("Profile"), "Profile", "👤"),
            (AdminTab::Placeholder("Authentication"), "Authentication", "🔐"),
            (AdminTab::Placeholder("Error Page"), "Error Page", "⚠️"),
            (AdminTab::Placeholder("Blank Page"), "Blank Page", "⬜"),
            (AdminTab::Placeholder("Pricing"), "Pricing", "💲"),
            (AdminTab::Placeholder("Coming Soon"), "Coming Soon", "⏳"),
            (AdminTab::Placeholder("Under Maintenance"), "Under Maintenance", "🛠️"),
        ]),
        ("Settings", vec![
            (AdminTab::Settings, "General Settings", "⚙️"),
            (AdminTab::Placeholder("Website Settings"), "Website Settings", "🌐"),
            (AdminTab::Placeholder("App Settings"), "App Settings", "📱"),
            (AdminTab::Placeholder("System Settings"), "System Settings", "🖥️"),
            (AdminTab::Placeholder("Financial Settings"), "Financial Settings", "💵"),
            (AdminTab::Placeholder("Other Settings"), "Other Settings", "🔧"),
            (AdminTab::Placeholder("Logout"), "Logout", "🚪"),
        ]),
    ];

    rsx! {
        if drawer_open {
            // Overlay backdrop (closes sidebar on click)
            div {
                class: "admin-sidebar-overlay",
                onclick: move |_| on_close.call(()),
            }
        }

        aside {
            class: if drawer_open { "admin-sidebar-drawer open" } else { "admin-sidebar-drawer" },
            
            div {
                class: "admin-sidebar-header",
                h3 { "DreamsPOS" }
                button {
                    class: "admin-sidebar-close",
                    onclick: move |_| on_close.call(()),
                    "✕"
                }
            }

            nav {
                class: "admin-sidebar-nav",
                for (group_label, items) in menu_groups {
                    div { class: "admin-sidebar-group",
                        h4 { class: "admin-sidebar-group-title", "{group_label}" }
                        for (tab, label, icon) in items {
                            button {
                                class: if current_tab == tab { "admin-sidebar-item active" } else { "admin-sidebar-item" },
                                aria_label: "{label}",
                                onclick: move |_| {
                                    on_select.call(tab);
                                    on_close.call(());
                                },
                                span { class: "admin-sidebar-icon", "{icon}" }
                                span { class: "admin-sidebar-label", "{label}" }
                            }
                        }
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
        AdminTab::SalesDashboard => ("Sales Dashboard", "Sales KPIs, trends, and recent performance."),
        AdminTab::Sales => ("Sales", "Review transactions and revenue performance."),
        AdminTab::SalesReport => ("Sales Report", "Summary of sales and performance trends."),
        AdminTab::Products => ("Products", "Manage inventory and product catalog."),
        AdminTab::CreateProduct => ("Create Product", "Add a new item to your catalog."),
        AdminTab::ExpiredProducts => ("Expired Products", "Track expired or near-expiry inventory."),
        AdminTab::LowStocks => ("Low Stocks", "Monitor products below reorder levels."),
        AdminTab::Categories => ("Category", "Organize product categories."),
        AdminTab::SubCategories => ("Sub Category", "Organize product subcategories."),
        AdminTab::Brands => ("Brands", "Manage product brands."),
        AdminTab::Units => ("Units", "Configure measurement units."),
        AdminTab::VariantAttributes => ("Variant Attributes", "Define product variants and options."),
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
        AdminTab::Access => ("Access Control", "Manage roles, permissions, and assignments."),
        AdminTab::Suppliers => ("Suppliers", "Coordinate supplier contacts and performance."),
        AdminTab::Shifts => ("Shifts", "Coordinate staffing coverage and schedules."),
        AdminTab::Settings => ("Settings", "Configure store preferences and system settings."),
        AdminTab::Placeholder(label) => (label, "Coming soon."),
    }
}

// `SidebarMenu` renders the floating admin drawer entries directly.
