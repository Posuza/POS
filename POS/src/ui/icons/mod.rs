use dioxus::prelude::*;
use dioxus_free_icons::IconShape;
use dioxus_free_icons::icons::fa_solid_icons::*;

fn render_icon(shape: impl IconShape, class: &str) -> Element {
    let vb = shape.view_box().to_string();
    let xmlns = shape.xmlns().to_string();
    let children = shape.child_elements();
    rsx! {
        svg {
            class: "{class}",
            view_box: "{vb}",
            xmlns: "{xmlns}",
            fill: "currentColor",
            height: "16",
            width: "16",
            {children}
        }
    }
}

#[component]
pub fn Icon(name: String, class: Option<String>, aria_label: Option<String>) -> Element {
    let cls = class.unwrap_or_default();
    let c = cls.as_str();
    match name.as_str() {
        "fa-home"                 => render_icon(FaHouse, c),
        "fa-compass"              => render_icon(FaCompass, c),
        "fa-box"                  => render_icon(FaBox, c),
        "fa-plus"                 => render_icon(FaPlus, c),
        "fa-clock"                => render_icon(FaClock, c),
        "fa-triangle-exclamation" => render_icon(FaTriangleExclamation, c),
        "fa-tag"                  => render_icon(FaTag, c),
        "fa-tags"                 => render_icon(FaTags, c),
        "fa-puzzle-piece"         => render_icon(FaPuzzlePiece, c),
        "fa-ruler"                => render_icon(FaRuler, c),
        "fa-dna"                  => render_icon(FaDna, c),
        "fa-shield"               => render_icon(FaShield, c),
        "fa-barcode"              => render_icon(FaBarcode, c),
        "fa-qrcode"               => render_icon(FaQrcode, c),
        "fa-screwdriver-wrench"   => render_icon(FaScrewdriverWrench, c),
        "fa-right-left"           => render_icon(FaRightLeft, c),
        "fa-chart-bar"            => render_icon(FaChartBar, c),
        "fa-money-bill"           => render_icon(FaMoneyBill, c),
        "fa-file-invoice"         => render_icon(FaFileInvoice, c),
        "fa-arrow-left"           => render_icon(FaArrowLeft, c),
        "fa-file-lines"           => render_icon(FaFileLines, c),
        "fa-cart-shopping"        => render_icon(FaCartShopping, c),
        "fa-gift"                 => render_icon(FaGift, c),
        "fa-percent"              => render_icon(FaPercent, c),
        "fa-basket-shopping"      => render_icon(FaBasketShopping, c),
        "fa-clipboard-list"       => render_icon(FaClipboardList, c),
        "fa-briefcase"            => render_icon(FaBriefcase, c),
        "fa-coins"                => render_icon(FaCoins, c),
        "fa-building-columns"     => render_icon(FaBuildingColumns, c),
        "fa-money-bill-transfer"  => render_icon(FaMoneyBillTransfer, c),
        "fa-file"                 => render_icon(FaFile, c),
        "fa-calculator"           => render_icon(FaCalculator, c),
        "fa-water"                => render_icon(FaWater, c),
        "fa-users"                => render_icon(FaUsers, c),
        "fa-truck"                => render_icon(FaTruck, c),
        "fa-store"                => render_icon(FaStore, c),
        "fa-warehouse"            => render_icon(FaWarehouse, c),
        "fa-user-group"           => render_icon(FaUserGroup, c),
        "fa-building"             => render_icon(FaBuilding, c),
        "fa-bullseye"             => render_icon(FaBullseye, c),
        "fa-thumbtack"            => render_icon(FaThumbtack, c),
        "fa-umbrella-beach"       => render_icon(FaUmbrellaBeach, c),
        "fa-champagne-glasses"    => render_icon(FaChampagneGlasses, c),
        "fa-chart-line"           => render_icon(FaChartLine, c),
        "fa-file-invoice-dollar"  => render_icon(FaFileInvoiceDollar, c),
        "fa-calendar"             => render_icon(FaCalendar, c),
        "fa-user"                 => render_icon(FaUser, c),
        "fa-shield-halved"        => render_icon(FaShieldHalved, c),
        "fa-trash"                => render_icon(FaTrash, c),
        "fa-newspaper"            => render_icon(FaNewspaper, c),
        "fa-location-dot"         => render_icon(FaLocationDot, c),
        "fa-comment-dots"         => render_icon(FaCommentDots, c),
        "fa-circle-question"      => render_icon(FaCircleQuestion, c),
        "fa-lock"                 => render_icon(FaLock, c),
        "fa-square"               => render_icon(FaSquare, c),
        "fa-dollar-sign"          => render_icon(FaDollarSign, c),
        "fa-hourglass"            => render_icon(FaHourglass, c),
        "fa-gear"                 => render_icon(FaGear, c),
        "fa-globe"                => render_icon(FaGlobe, c),
        "fa-mobile-screen"        => render_icon(FaMobileScreen, c),
        "fa-desktop"              => render_icon(FaDesktop, c),
        "fa-wrench"               => render_icon(FaWrench, c),
        "fa-right-from-bracket"   => render_icon(FaRightFromBracket, c),
        _                         => rsx! { span { "?" } },
    }
}
