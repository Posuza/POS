/// Styles module

/// Return combined styles as a compile-time static string to avoid
/// allocating/joining on every call (helps rendering performance).
pub fn get_styles() -> &'static str {
    concat!(
        include_str!("../styles/main.css"),
        "\n",
        include_str!("../styles/base.css"),
        "\n",
        include_str!("../styles/pos.css"),
        "\n",
        include_str!("../styles/admin.css"),
        "\n",
        include_str!("../styles/login.css"),
        "\n",
        include_str!("../styles/cards.css"),
        "\n",
        include_str!("../styles/forms.css"),
        "\n",
        include_str!("../styles/modals.css"),
        "\n",
        include_str!("../styles/navbar.css"),
        "\n",
        include_str!("../styles/components/buttons.css"),
        "\n",
        include_str!("../styles/components/sidebar.css"),
        "\n",
        include_str!("../styles/components/dashboards/tables.css"),
        "\n",
        include_str!("../styles/components/dashboards/sales_trend.css"),
        "\n",
        include_str!("../styles/components/dashboards/settings.css"),
        "\n",
    )
}
