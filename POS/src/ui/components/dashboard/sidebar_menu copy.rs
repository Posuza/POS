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

#[derive(Clone, PartialEq)]
struct MenuItem {
    tab: AdminTab,
    name: &'static str,
    icon: &'static str,
}

#[derive(Clone, PartialEq)]
struct MenuSection {
    name: &'static str,
    icon: &'static str,
    items: Vec<MenuItem>,
}

fn build_menu() -> Vec<MenuSection> {
    vec![
        MenuSection { name: "Main", icon: "fa-home", items: vec![
            MenuItem { tab: AdminTab::Dashboard,  name: "Dashboard",   icon: "fa-home" },
            MenuItem { tab: AdminTab::SuperAdmin, name: "Super Admin", icon: "fa-compass" },
        ]},
        MenuSection { name: "Inventory", icon: "fa-box", items: vec![
            MenuItem { tab: AdminTab::Products,          name: "Products",           icon: "fa-box" },
            MenuItem { tab: AdminTab::CreateProduct,     name: "Create Product",     icon: "fa-plus" },
            MenuItem { tab: AdminTab::ExpiredProducts,   name: "Expired Products",   icon: "fa-clock" },
            MenuItem { tab: AdminTab::LowStocks,         name: "Low Stocks",         icon: "fa-triangle-exclamation" },
            MenuItem { tab: AdminTab::Categories,        name: "Category",           icon: "fa-tag" },
            MenuItem { tab: AdminTab::SubCategories,     name: "Sub Category",       icon: "fa-puzzle-piece" },
            MenuItem { tab: AdminTab::Brands,            name: "Brands",             icon: "fa-tags" },
            MenuItem { tab: AdminTab::Units,             name: "Units",              icon: "fa-ruler" },
            MenuItem { tab: AdminTab::VariantAttributes, name: "Variant Attributes", icon: "fa-dna" },
            MenuItem { tab: AdminTab::Warranties,        name: "Warranties",         icon: "fa-shield" },
            MenuItem { tab: AdminTab::PrintBarcode,      name: "Print Barcode",      icon: "fa-barcode" },
            MenuItem { tab: AdminTab::PrintQrCode,       name: "Print QR Code",      icon: "fa-qrcode" },
        ]},
        MenuSection { name: "Stock", icon: "fa-right-left", items: vec![
            MenuItem { tab: AdminTab::ManageStock,     name: "Manage Stock",     icon: "fa-box" },
            MenuItem { tab: AdminTab::StockAdjustment, name: "Stock Adjustment", icon: "fa-screwdriver-wrench" },
            MenuItem { tab: AdminTab::StockTransfer,   name: "Stock Transfer",   icon: "fa-right-left" },
        ]},
        MenuSection { name: "Sales", icon: "fa-money-bill", items: vec![
            MenuItem { tab: AdminTab::SalesDashboard,              name: "Sales Dashboard", icon: "fa-chart-bar" },
            MenuItem { tab: AdminTab::Sales,                       name: "Sales",           icon: "fa-money-bill" },
            MenuItem { tab: AdminTab::Placeholder("Invoices"),     name: "Invoices",        icon: "fa-file-invoice" },
            MenuItem { tab: AdminTab::Placeholder("Sales Return"), name: "Sales Return",    icon: "fa-arrow-left" },
            MenuItem { tab: AdminTab::Placeholder("Quotation"),    name: "Quotation",       icon: "fa-file-lines" },
            MenuItem { tab: AdminTab::Placeholder("POS"),          name: "POS",             icon: "fa-cart-shopping" },
        ]},
        MenuSection { name: "Promo", icon: "fa-tags", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Coupons"),   name: "Coupons",   icon: "fa-tags" },
            MenuItem { tab: AdminTab::Placeholder("Gift Card"), name: "Gift Card", icon: "fa-gift" },
            MenuItem { tab: AdminTab::Placeholder("Discount"),  name: "Discount",  icon: "fa-percent" },
        ]},
        MenuSection { name: "Purchases", icon: "fa-basket-shopping", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Purchases"),       name: "Purchases",       icon: "fa-basket-shopping" },
            MenuItem { tab: AdminTab::Placeholder("Purchase Order"),  name: "Purchase Order",  icon: "fa-clipboard-list" },
            MenuItem { tab: AdminTab::Placeholder("Purchase Return"), name: "Purchase Return", icon: "fa-arrow-left" },
        ]},
        MenuSection { name: "Finance & Accounts", icon: "fa-coins", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Expenses"),          name: "Expenses",          icon: "fa-briefcase" },
            MenuItem { tab: AdminTab::Placeholder("Income"),            name: "Income",            icon: "fa-coins" },
            MenuItem { tab: AdminTab::Placeholder("Bank Accounts"),     name: "Bank Accounts",     icon: "fa-building-columns" },
            MenuItem { tab: AdminTab::Placeholder("Money Transfer"),    name: "Money Transfer",    icon: "fa-money-bill-transfer" },
            MenuItem { tab: AdminTab::Placeholder("Balance Sheet"),     name: "Balance Sheet",     icon: "fa-file" },
            MenuItem { tab: AdminTab::Placeholder("Trial Balance"),     name: "Trial Balance",     icon: "fa-calculator" },
            MenuItem { tab: AdminTab::Placeholder("Cash Flow"),         name: "Cash Flow",         icon: "fa-water" },
            MenuItem { tab: AdminTab::Placeholder("Account Statement"), name: "Account Statement", icon: "fa-file-lines" },
        ]},
        MenuSection { name: "People", icon: "fa-users", items: vec![
            MenuItem { tab: AdminTab::Customers,  name: "Customers",  icon: "fa-users" },
            MenuItem { tab: AdminTab::Billers,    name: "Billers",    icon: "fa-file-invoice" },
            MenuItem { tab: AdminTab::Suppliers,  name: "Suppliers",  icon: "fa-truck" },
            MenuItem { tab: AdminTab::Stores,     name: "Stores",     icon: "fa-store" },
            MenuItem { tab: AdminTab::Warehouses, name: "Warehouses", icon: "fa-warehouse" },
        ]},
        MenuSection { name: "HRM", icon: "fa-user-group", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Employees"),   name: "Employees",   icon: "fa-user-group" },
            MenuItem { tab: AdminTab::Placeholder("Departments"), name: "Departments", icon: "fa-building" },
            MenuItem { tab: AdminTab::Placeholder("Designation"), name: "Designation", icon: "fa-bullseye" },
            MenuItem { tab: AdminTab::Shifts,                     name: "Shifts",      icon: "fa-clock" },
            MenuItem { tab: AdminTab::Placeholder("Attendance"),  name: "Attendance",  icon: "fa-thumbtack" },
            MenuItem { tab: AdminTab::Placeholder("Leaves"),      name: "Leaves",      icon: "fa-umbrella-beach" },
            MenuItem { tab: AdminTab::Placeholder("Holidays"),    name: "Holidays",    icon: "fa-champagne-glasses" },
            MenuItem { tab: AdminTab::Placeholder("Payroll"),     name: "Payroll",     icon: "fa-money-bill" },
        ]},
        MenuSection { name: "Reports", icon: "fa-chart-line", items: vec![
            MenuItem { tab: AdminTab::SalesReport,                   name: "Sales Report",     icon: "fa-chart-line" },
            MenuItem { tab: AdminTab::PurchaseReport,                name: "Purchase Report",  icon: "fa-file-invoice" },
            MenuItem { tab: AdminTab::InventoryReport,               name: "Inventory Report", icon: "fa-box" },
            MenuItem { tab: AdminTab::InvoiceReport,                 name: "Invoice Report",   icon: "fa-file-invoice" },
            MenuItem { tab: AdminTab::SupplierReport,                name: "Supplier Report",  icon: "fa-truck" },
            MenuItem { tab: AdminTab::CustomerReport,                name: "Customer Report",  icon: "fa-users" },
            MenuItem { tab: AdminTab::ProductReport,                 name: "Product Report",   icon: "fa-chart-bar" },
            MenuItem { tab: AdminTab::Placeholder("Expense Report"), name: "Expense Report",   icon: "fa-briefcase" },
            MenuItem { tab: AdminTab::Placeholder("Income Report"),  name: "Income Report",    icon: "fa-coins" },
            MenuItem { tab: AdminTab::Placeholder("Tax Report"),     name: "Tax Report",       icon: "fa-file-invoice-dollar" },
            MenuItem { tab: AdminTab::Placeholder("Profit & Loss"),  name: "Profit & Loss",    icon: "fa-chart-line" },
            MenuItem { tab: AdminTab::Placeholder("Annual Report"),  name: "Annual Report",    icon: "fa-calendar" },
        ]},
        MenuSection { name: "User Management", icon: "fa-shield-halved", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Users"),                  name: "Users",                  icon: "fa-user" },
            MenuItem { tab: AdminTab::Access,                                name: "Roles & Permissions",    icon: "fa-shield-halved" },
            MenuItem { tab: AdminTab::Placeholder("Delete Account Request"), name: "Delete Account Request", icon: "fa-trash" },
        ]},
        MenuSection { name: "Content (CMS)", icon: "fa-newspaper", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Pages"),        name: "Pages",        icon: "fa-file" },
            MenuItem { tab: AdminTab::Placeholder("Blog"),         name: "Blog",         icon: "fa-newspaper" },
            MenuItem { tab: AdminTab::Placeholder("Location"),     name: "Location",     icon: "fa-location-dot" },
            MenuItem { tab: AdminTab::Placeholder("Testimonials"), name: "Testimonials", icon: "fa-comment-dots" },
            MenuItem { tab: AdminTab::Placeholder("FAQ"),          name: "FAQ",          icon: "fa-circle-question" },
        ]},
        MenuSection { name: "Pages", icon: "fa-file", items: vec![
            MenuItem { tab: AdminTab::Placeholder("Profile"),           name: "Profile",           icon: "fa-user" },
            MenuItem { tab: AdminTab::Placeholder("Authentication"),    name: "Authentication",    icon: "fa-lock" },
            MenuItem { tab: AdminTab::Placeholder("Error Page"),        name: "Error Page",        icon: "fa-triangle-exclamation" },
            MenuItem { tab: AdminTab::Placeholder("Blank Page"),        name: "Blank Page",        icon: "fa-square" },
            MenuItem { tab: AdminTab::Placeholder("Pricing"),           name: "Pricing",           icon: "fa-dollar-sign" },
            MenuItem { tab: AdminTab::Placeholder("Coming Soon"),       name: "Coming Soon",       icon: "fa-hourglass" },
            MenuItem { tab: AdminTab::Placeholder("Under Maintenance"), name: "Under Maintenance", icon: "fa-screwdriver-wrench" },
        ]},
        MenuSection { name: "Settings", icon: "fa-gear", items: vec![
            MenuItem { tab: AdminTab::Settings,                         name: "General Settings",   icon: "fa-gear" },
            MenuItem { tab: AdminTab::Placeholder("Website Settings"),  name: "Website Settings",   icon: "fa-globe" },
            MenuItem { tab: AdminTab::Placeholder("App Settings"),      name: "App Settings",       icon: "fa-mobile-screen" },
            MenuItem { tab: AdminTab::Placeholder("System Settings"),   name: "System Settings",    icon: "fa-desktop" },
            MenuItem { tab: AdminTab::Placeholder("Financial Settings"),name: "Financial Settings", icon: "fa-money-bill" },
            MenuItem { tab: AdminTab::Placeholder("Other Settings"),    name: "Other Settings",     icon: "fa-wrench" },
            MenuItem { tab: AdminTab::Placeholder("Logout"),            name: "Logout",             icon: "fa-right-from-bracket" },
        ]},
    ]
}

#[component]
pub fn SidebarMenu(
    active_tab: Signal<AdminTab>,
    on_select: EventHandler<AdminTab>,
    is_open: Signal<bool>,
    on_close: EventHandler<()>,
) -> Element {
    let drawer_open = *is_open.read();
    let current_tab = *active_tab.read();
    let mut collapsed: Signal<Vec<bool>> = use_signal(|| vec![false; 14]);
    let menu = build_menu();
    let active_flags: Vec<bool> = menu.iter().map(|s| s.items.iter().any(|i| i.tab == current_tab)).collect();
    let collapse_states: Vec<bool> = collapsed.read().clone();
    drop(collapsed.read());

    rsx! {
        if drawer_open {
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
                    crate::ui::icons::Icon { name: "fa-chevron-left".to_string(), class: Some("admin-sidebar-close-icon".to_string()), aria_label: Some("Close".to_string()) }
                }
            }
            div {
                class: "admin-sidebar-nav",
                for (idx, section) in menu.into_iter().enumerate() {
                    SidebarGroup {
                        idx,
                        name: section.name,
                        icon: section.icon,
                        items: section.items,
                        is_collapsed: collapse_states[idx],
                        current_tab,
                        has_active: active_flags[idx],
                        on_toggle: move |i: usize| { let v = collapsed.read()[i]; collapsed.write()[i] = !v; },
                        on_select,
                        on_close,
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarGroup(
    idx: usize,
    name: &'static str,
    icon: &'static str,
    items: Vec<MenuItem>,
    is_collapsed: bool,
    has_active: bool,
    current_tab: AdminTab,
    on_toggle: EventHandler<usize>,
    on_select: EventHandler<AdminTab>,
    on_close: EventHandler<()>,
) -> Element {
    let section_class = if has_active { "admin-sidebar-group section-active" } else { "admin-sidebar-group" };
    let section_title_class = if has_active { "admin-sidebar-group-title title-active" } else { "admin-sidebar-group-title" };
    rsx! {
            div { class: "{section_class}",
                div {
                    class: "{section_title_class}",
                    onclick: move |_| on_toggle.call(idx),
                    div {
                        class: "section_title_left",
                        crate::ui::icons::Icon { name: icon.to_string(), class: Some("admin-sidebar-group-icon".to_string()), aria_label: None }
                        span { "{name}" }
                    }
                    crate::ui::icons::Icon {
                        name: if is_collapsed { "fa-chevron-right" } else { "fa-chevron-down" }.to_string(),
                        class: Some("admin-sidebar-chevron".to_string()),
                        aria_label: None,
                    }
                }
            // if !is_collapsed {
            //     div { class: "admin-sidebar-items-container",
            //         for item in items.into_iter() {
            //             button {
            //                 class: if current_tab == item.tab { "admin-sidebar-item active" } else { "admin-sidebar-item" },
            //                 aria_label: "{item.name}",
            //                 onclick: move |_| {
            //                     on_select.call(item.tab);
            //                     on_close.call(());
            //                 },
            //                 crate::ui::icons::Icon { name: item.icon.to_string(), class: Some("admin-sidebar-icon".to_string()), aria_label: Some(item.name.to_string()) }
            //                 span { class: "admin-sidebar-label", "{item.name}" }
            //             }
            //         }
            //     }
            // }
        }
    }
}
