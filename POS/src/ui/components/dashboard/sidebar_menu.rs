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