use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AdminTab {
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
pub fn SidebarMenu(
    active_tab: Signal<AdminTab>,
    on_select: EventHandler<AdminTab>,
    is_open: Signal<bool>,
    on_close: EventHandler<()>
) -> Element {
    let drawer_open = *is_open.read();
    let current_tab = *active_tab.read();
    let section_icons: Vec<(&str, &str)> = vec![
        ("Main", "fa-home"),
        ("Inventory", "fa-coins"),
        ("Stock", "fa-boxes"),
        ("Sales", "fa-money-bill"),
        ("Promo", "fa-tags"),
        ("Purchases", "fa-basket-shopping"),
        ("Finance & Accounts", "fa-file-invoice-dollar"),
        ("People", "fa-users"),
        ("HRM", "fa-user-group"),
        ("Reports", "fa-chart-line"),
        ("User Management", "fa-shield-halved"),
        ("Content (CMS)", "fa-newspaper"),
        ("Pages", "fa-file"),
        ("Settings", "fa-gear")
    ];
    let menu_groups: Vec<(&'static str, Vec<(AdminTab, &'static str, &'static str)>)> = vec![
        (
            "Main",
            vec![
                (AdminTab::Dashboard, "Dashboard", "fa-home"),
                (AdminTab::SuperAdmin, "Super Admin", "fa-compass")
            ],
        ),
        (
            "Inventory",
            vec![
                (AdminTab::Products, "Products", "fa-box"),
                (AdminTab::CreateProduct, "Create Product", "fa-plus"),
                (AdminTab::ExpiredProducts, "Expired Products", "fa-clock"),
                (AdminTab::LowStocks, "Low Stocks", "fa-triangle-exclamation"),
                (AdminTab::Categories, "Category", "fa-tag"),
                (AdminTab::SubCategories, "Sub Category", "fa-puzzle-piece"),
                (AdminTab::Brands, "Brands", "fa-tags"),
                (AdminTab::Units, "Units", "fa-ruler"),
                (AdminTab::VariantAttributes, "Variant Attributes", "fa-dna"),
                (AdminTab::Warranties, "Warranties", "fa-shield"),
                (AdminTab::PrintBarcode, "Print Barcode", "fa-barcode"),
                (AdminTab::PrintQrCode, "Print QR Code", "fa-qrcode")
            ],
        ),
        (
            "Stock",
            vec![
                (AdminTab::ManageStock, "Manage Stock", "fa-box"),
                (AdminTab::StockAdjustment, "Stock Adjustment", "fa-screwdriver-wrench"),
                (AdminTab::StockTransfer, "Stock Transfer", "fa-right-left")
            ],
        ),
        (
            "Sales",
            vec![
                (AdminTab::SalesDashboard, "Sales Dashboard", "fa-chart-bar"),
                (AdminTab::Sales, "Sales", "fa-money-bill"),
                (AdminTab::Placeholder("Invoices"), "Invoices", "fa-file-invoice"),
                (AdminTab::Placeholder("Sales Return"), "Sales Return", "fa-arrow-left"),
                (AdminTab::Placeholder("Quotation"), "Quotation", "fa-file-lines"),
                (AdminTab::Placeholder("POS"), "POS", "fa-cart-shopping")
            ],
        ),
        (
            "Promo",
            vec![
                (AdminTab::Placeholder("Coupons"), "Coupons", "fa-tags"),
                (AdminTab::Placeholder("Gift Card"), "Gift Card", "fa-gift"),
                (AdminTab::Placeholder("Discount"), "Discount", "fa-percent")
            ],
        ),
        (
            "Purchases",
            vec![
                (AdminTab::Placeholder("Purchases"), "Purchases", "fa-basket-shopping"),
                (AdminTab::Placeholder("Purchase Order"), "Purchase Order", "fa-clipboard-list"),
                (AdminTab::Placeholder("Purchase Return"), "Purchase Return", "fa-arrow-left")
            ],
        ),
        (
            "Finance & Accounts",
            vec![
                (AdminTab::Placeholder("Expenses"), "Expenses", "fa-briefcase"),
                (AdminTab::Placeholder("Income"), "Income", "fa-coins"),
                (AdminTab::Placeholder("Bank Accounts"), "Bank Accounts", "fa-building-columns"),
                (
                    AdminTab::Placeholder("Money Transfer"),
                    "Money Transfer",
                    "fa-money-bill-transfer",
                ),
                (AdminTab::Placeholder("Balance Sheet"), "Balance Sheet", "fa-file"),
                (AdminTab::Placeholder("Trial Balance"), "Trial Balance", "fa-calculator"),
                (AdminTab::Placeholder("Cash Flow"), "Cash Flow", "fa-water"),
                (AdminTab::Placeholder("Account Statement"), "Account Statement", "fa-file-lines")
            ],
        ),
        (
            "People",
            vec![
                (AdminTab::Customers, "Customers", "fa-users"),
                (AdminTab::Billers, "Billers", "fa-file-invoice"),
                (AdminTab::Suppliers, "Suppliers", "fa-truck"),
                (AdminTab::Stores, "Stores", "fa-store"),
                (AdminTab::Warehouses, "Warehouses", "fa-warehouse")
            ],
        ),
        (
            "HRM",
            vec![
                (AdminTab::Placeholder("Employees"), "Employees", "fa-user-group"),
                (AdminTab::Placeholder("Departments"), "Departments", "fa-building"),
                (AdminTab::Placeholder("Designation"), "Designation", "fa-bullseye"),
                (AdminTab::Shifts, "Shifts", "fa-clock"),
                (AdminTab::Placeholder("Attendance"), "Attendance", "fa-thumbtack"),
                (AdminTab::Placeholder("Leaves"), "Leaves", "fa-umbrella-beach"),
                (AdminTab::Placeholder("Holidays"), "Holidays", "fa-champagne-glasses"),
                (AdminTab::Placeholder("Payroll"), "Payroll", "fa-money-bill")
            ],
        ),
        (
            "Reports",
            vec![
                (AdminTab::SalesReport, "Sales Report", "fa-chart-line"),
                (AdminTab::PurchaseReport, "Purchase Report", "fa-file-invoice"),
                (AdminTab::InventoryReport, "Inventory Report", "fa-box"),
                (AdminTab::InvoiceReport, "Invoice Report", "fa-file-invoice"),
                (AdminTab::SupplierReport, "Supplier Report", "fa-truck"),
                (AdminTab::CustomerReport, "Customer Report", "fa-users"),
                (AdminTab::ProductReport, "Product Report", "fa-chart-bar"),
                (AdminTab::Placeholder("Expense Report"), "Expense Report", "fa-briefcase"),
                (AdminTab::Placeholder("Income Report"), "Income Report", "fa-coins"),
                (AdminTab::Placeholder("Tax Report"), "Tax Report", "fa-file-invoice-dollar"),
                (AdminTab::Placeholder("Profit & Loss"), "Profit & Loss", "fa-chart-line"),
                (AdminTab::Placeholder("Annual Report"), "Annual Report", "fa-calendar")
            ],
        ),
        (
            "User Management",
            vec![
                (AdminTab::Placeholder("Users"), "Users", "fa-user"),
                (AdminTab::Access, "Roles & Permissions", "fa-shield-halved"),
                (
                    AdminTab::Placeholder("Delete Account Request"),
                    "Delete Account Request",
                    "fa-trash",
                )
            ],
        ),
        (
            "Content (CMS)",
            vec![
                (AdminTab::Placeholder("Pages"), "Pages", "fa-file"),
                (AdminTab::Placeholder("Blog"), "Blog", "fa-newspaper"),
                (AdminTab::Placeholder("Location"), "Location", "fa-location-dot"),
                (AdminTab::Placeholder("Testimonials"), "Testimonials", "fa-comment-dots"),
                (AdminTab::Placeholder("FAQ"), "FAQ", "fa-circle-question")
            ],
        ),
        (
            "Pages",
            vec![ 
                (AdminTab::Placeholder("Profile"), "Profile", "fa-user"),
                (AdminTab::Placeholder("Authentication"), "Authentication", "fa-lock"),
                (AdminTab::Placeholder("Error Page"), "Error Page", "fa-triangle-exclamation"),
                (AdminTab::Placeholder("Blank Page"), "Blank Page", "fa-square"),
                (AdminTab::Placeholder("Pricing"), "Pricing", "fa-dollar-sign"),
                (AdminTab::Placeholder("Coming Soon"), "Coming Soon", "fa-hourglass"),
                (
                    AdminTab::Placeholder("Under Maintenance"),
                    "Under Maintenance",
                    "fa-screwdriver-wrench",
                )
            ],
        ),
        (
            "Settings",
            vec![
                (AdminTab::Settings, "General Settings", "fa-gear"),
                (AdminTab::Placeholder("Website Settings"), "Website Settings", "fa-globe"),
                (AdminTab::Placeholder("App Settings"), "App Settings", "fa-mobile-screen"),
                (AdminTab::Placeholder("System Settings"), "System Settings", "fa-desktop"),
                (
                    AdminTab::Placeholder("Financial Settings"),
                    "Financial Settings",
                    "fa-money-bill",
                ),
                (AdminTab::Placeholder("Other Settings"), "Other Settings", "fa-wrench"),
                (AdminTab::Placeholder("Logout"), "Logout", "fa-right-from-bracket")
            ],
        )
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
                for (idx, (group_label, items)) in menu_groups.iter().enumerate() {
                    let section_icon = section_icons.iter().find(|(label, _)| *label == *group_label).map(|(_, icon)| icon).unwrap_or(&"fa-folder");
                    let is_open = true; // For now, always open. Replace with collapse state if needed.
                    div { class: "admin-sidebar-group",
                        h4 { class: "admin-sidebar-group-title",
                            style: "display: flex; align-items: center; justify-content: space-between; gap: 8px;",
                            crate::ui::icons::Icon {
                                name: section_icon.to_string(),
                                class: Some("admin-sidebar-icon-section".to_string()),
                                aria_label: Some(group_label.to_string()),
                            }
                            span { style: "flex: 1; text-align: left;", group_label }
                            crate::ui::icons::Icon {
                                name: if is_open { "fa-chevron-down" } else { "fa-chevron-right" }.to_string(),
                                class: Some("admin-sidebar-chevron".to_string()),
                                aria_label: Some("Toggle group".to_string()),
                                style: "margin-left: auto;",
                            }
                        }
                        for (tab, label, icon) in items {
                            button {
                                class: if current_tab == *tab { "admin-sidebar-item active" } else { "admin-sidebar-item" },
                                aria_label: label,
                                onclick: move |_| {
                                    on_select.call(*tab);
                                    on_close.call(());
                                },
                                crate::ui::icons::Icon {
                                    name: icon.to_string(),
                                    class: Some("admin-sidebar-icon".to_string()),
                                    aria_label: Some(label.to_string()),
                                }
                                span { class: "admin-sidebar-label", label }
                            }
                        }
                    }
                }
            }
        }
