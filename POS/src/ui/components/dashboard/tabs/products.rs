use super::prelude::*;

#[component]
pub(crate) fn ProductsTab() -> Element {
    let mut show_add_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut edit_product_id = use_signal(|| None::<String>);
    let mut product_query = use_signal(|| String::new());
    let mut product_sort = use_signal(|| "name".to_string());
    let mut product_order = use_signal(|| "asc".to_string());
    let mut product_page = use_signal(|| 1usize);
    let mut product_name = use_signal(|| String::new());
    let mut product_barcode = use_signal(|| String::new());
    let mut product_category = use_signal(|| String::new());
    let mut product_price = use_signal(|| String::new());
    let mut product_stock = use_signal(|| String::new());
    let product_image_preview = use_signal(|| None::<String>);
    let product_image_payload = use_signal(|| None::<String>);
    let product_image_type = use_signal(|| None::<String>);
    let mut edit_product_name = use_signal(|| String::new());
    let mut edit_product_barcode = use_signal(|| String::new());
    let mut edit_product_category = use_signal(|| String::new());
    let mut edit_product_price = use_signal(|| String::new());
    let mut edit_product_stock = use_signal(|| String::new());
    let edit_product_image_preview = use_signal(|| None::<String>);
    let edit_product_image_payload = use_signal(|| None::<String>);
    let mut edit_product_image_name = use_signal(|| None::<String>);
    let mut edit_product_image_type = use_signal(|| None::<String>);
    let store = get_store_fresh();
    let mut products_state = use_signal(|| store.products.clone());
    let mut edit_save_msg = use_signal(|| false);
    let mut edit_error = use_signal(|| None::<String>);
    let mut add_save_msg = use_signal(|| false);
    let mut add_error = use_signal(|| None::<String>);
    let mut delete_msg = use_signal(|| None::<String>);
    let mut delete_error = use_signal(|| None::<String>);
    let mut product_export_msg = use_signal(|| None::<String>);
    let mut product_export_error = use_signal(|| None::<String>);
    let products = products_state.read().clone();
    let users = &store.users;
    let sales = &store.sales;

    let total_products = products.len();
    let total_stock: i32 = products.iter().map(|p| p.quantity).sum();
    let active_users = users.iter().filter(|u| u.status == "active").count();
    let inactive_users = users.iter().filter(|u| u.status != "active").count();
    let total_transactions = sales.len();
    let total_revenue: f32 = sales.iter().map(|s| s.total).sum();
    let average_ticket = if total_transactions == 0 {
        0.0
    } else {
        total_revenue / total_transactions as f32
    };

    let total_products_s = format!("{}", total_products);
    let total_stock_s = format!("{}", total_stock);
    let active_users_s = format!("{}", active_users);
    let inactive_users_s = format!("{}", inactive_users);
    let total_transactions_s = format!("{} transactions", total_transactions);
    let total_revenue_s = format_price(total_revenue);
    let average_ticket_s = format!("{} avg ticket", format_price(average_ticket));
    let last_updated = sales
        .last()
        .map(|s| format_datetime(&s.created_at))
        .unwrap_or_else(|| "No recent updates".to_string());
    
    let handle_add_product = {
        let mut products_state = products_state.clone();
        let mut add_error = add_error.clone();
        let mut add_save_msg = add_save_msg.clone();
        let mut show_add_modal = show_add_modal.clone();
        let mut product_name = product_name.clone();
        let mut product_barcode = product_barcode.clone();
        let mut product_category = product_category.clone();
        let mut product_price = product_price.clone();
        let mut product_stock = product_stock.clone();
        let mut product_image_preview = product_image_preview.clone();
        let mut product_image_payload = product_image_payload.clone();
        let mut product_image_type = product_image_type.clone();
        move |_| {
            if product_name.read().is_empty() || product_barcode.read().is_empty() {
                add_error.set(Some("Product name and barcode are required.".to_string()));
                return;
            }
            let price = match product_price.read().parse::<f32>() {
                Ok(v) if v >= 0.01 => v,
                _ => {
                    add_error.set(Some("Price must be at least 0.01.".to_string()));
                    return;
                }
            };
            let stock = match product_stock.read().parse::<i32>() {
                Ok(v) if v >= 0 => v,
                _ => {
                    add_error.set(Some("Stock must be a non-negative integer.".to_string()));
                    return;
                }
            };
            let mut updated = products_state.read().clone();
            if updated.iter().any(|p| p.barcode == *product_barcode.read()) {
                add_error.set(Some("A product with this barcode already exists.".to_string()));
                return;
            }
            let mut image_filename: Option<String> = None;
            let mut image_type: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (product_image_payload.read().clone(), product_image_type.read().clone()) {
                match ImageService::save_product_image(&payload, &img_type) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type = Some(img_type);
                    }
                    Err(err) => {
                        add_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            let product = Product {
                id: new_id("prod"),
                barcode: product_barcode.read().clone(),
                name: product_name.read().clone(),
                description: None,
                price,
                quantity: stock,
                category: if product_category.read().is_empty() { "Uncategorized".to_string() } else { product_category.read().clone() },
                product_image: image_filename,
                product_image_type: image_type,
                created_at: now.clone(),
                updated_at: now,
            };
            updated.push(product);
            match save_products_to_json(&updated) {
                Ok(_) => {
                    products_state.set(updated);
                    add_error.set(None);
                    add_save_msg.set(true);
                    product_name.set(String::new());
                    product_barcode.set(String::new());
                    product_category.set(String::new());
                    product_price.set(String::new());
                    product_stock.set(String::new());
                    product_image_preview.set(None);
                    product_image_payload.set(None);
                    product_image_type.set(None);
                    show_add_modal.set(false);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        add_save_msg.set(false);
                    });
                }
                Err(err) => add_error.set(Some(err)),
            }
        }
    };
    
    // Render rows from JSON products inline (rsx expects an iterator)
    let mut products_sorted: Vec<_> = products.iter().cloned().collect();
    products_sorted.sort_by(|a, b| {
        let a_val = a.price * a.quantity as f32;
        let b_val = b.price * b.quantity as f32;
        b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
    });
    let low_stock_items: Vec<_> = products
        .iter()
        .filter(|p| p.quantity < LOW_STOCK_THRESHOLD)
        .cloned()
        .collect();
    let out_of_stock = products.iter().filter(|p| p.quantity == 0).count();
    let low_stock_count = low_stock_items.len();

    let handle_edit_product = move |_| {
        let id = match edit_product_id.read().clone() {
            Some(id) => id,
            None => return,
        };
        let price = edit_product_price.read().parse::<f32>();
        let stock = edit_product_stock.read().parse::<i32>();
        if price.is_err() || stock.is_err() {
            edit_error.set(Some("Invalid price or stock value.".to_string()));
            return;
        }
        let price = price.unwrap_or(0.0);
        let stock = stock.unwrap_or(0);
        if price < 0.01 {
            edit_error.set(Some("Price must be at least 0.01.".to_string()));
            return;
        }
        if stock < 0 {
            edit_error.set(Some("Stock must be a non-negative integer.".to_string()));
            return;
        }

        let mut updated = products_state.read().clone();
        if updated.iter().any(|p| p.barcode == *edit_product_barcode.read() && p.id != id) {
            edit_error.set(Some("Another product already uses this barcode.".to_string()));
            return;
        }
        let mut image_filename = edit_product_image_name.read().clone();
        let mut image_type = edit_product_image_type.read().clone();
        if let (Some(payload), Some(img_type)) = (edit_product_image_payload.read().clone(), edit_product_image_type.read().clone()) {
            match ImageService::save_product_image(&payload, &img_type) {
                Ok(filename) => {
                    image_filename = Some(filename);
                    image_type = Some(img_type);
                }
                Err(err) => {
                    edit_error.set(Some(format!("Image upload failed: {}", err)));
                    return;
                }
            }
        }
        if let Some(p) = updated.iter_mut().find(|p| p.id == id) {
            p.name = edit_product_name.read().clone();
            p.barcode = edit_product_barcode.read().clone();
            p.category = edit_product_category.read().clone();
            p.price = price;
            p.quantity = stock;
            p.product_image = image_filename;
            p.product_image_type = image_type;
        } else {
            edit_error.set(Some("Product not found for update.".to_string()));
            return;
        }

        match save_products_to_json(&updated) {
            Ok(_) => {
                products_state.set(updated);
                edit_error.set(None);
                edit_save_msg.set(true);
                show_edit_modal.set(false);
                edit_product_id.set(None);
                spawn(async move {
                    sleep_ms(2_000).await;
                    edit_save_msg.set(false);
                });
            }
            Err(err) => {
                edit_error.set(Some(err));
            }
        }
    };

    let query = product_query.read().to_lowercase();
    let mut filtered_products: Vec<Product> = products
        .clone()
        .into_iter()
        .filter(|p| {
            if query.is_empty() {
                true
            } else {
                let name = p.name.to_lowercase();
                let barcode = p.barcode.to_lowercase();
                let category = p.category.to_lowercase();
                name.contains(&query) || barcode.contains(&query) || category.contains(&query)
            }
        })
        .collect();
    let sort_key = product_sort.read().clone();
    filtered_products.sort_by(|a, b| {
        match sort_key.as_str() {
            "stock" => a.quantity.cmp(&b.quantity),
            "price" => a.price.partial_cmp(&b.price).unwrap_or(std::cmp::Ordering::Equal),
            "value" => {
                let av = a.price * a.quantity as f32;
                let bv = b.price * b.quantity as f32;
                av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
            }
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    if product_order.read().as_str() == "desc" {
        filtered_products.reverse();
    }
    let page_size = 10usize;
    let total_pages = std::cmp::max(1, (filtered_products.len() + page_size - 1) / page_size);
    let current_page = (*product_page.read()).min(total_pages).max(1);
    let start = (current_page - 1) * page_size;
    let end = std::cmp::min(start + page_size, filtered_products.len());
    let page_items: Vec<Product> = if filtered_products.is_empty() {
        Vec::new()
    } else {
        filtered_products[start..end].to_vec()
    };

    let mut product_rows: Vec<Element> = Vec::new();
    for p in page_items.into_iter() {
        let status = if p.quantity == 0 { "Out" } else if p.quantity < LOW_STOCK_THRESHOLD { "Low" } else { "OK" };
        let status_class = if p.quantity == 0 { "status-chip danger" } else if p.quantity < LOW_STOCK_THRESHOLD { "status-chip warning" } else { "status-chip ok" };
        let pid = p.id.clone();
        let pname = p.name.clone();
        let pbarcode = p.barcode.clone();
        let pcategory = p.category.clone();
        let pprice = p.price;
        let pqty = p.quantity;
        let pimage_name = p.product_image.clone();
        let pimage_type = p.product_image_type.clone();
        let mut edit_product_id = edit_product_id.clone();
        let mut edit_product_name = edit_product_name.clone();
        let mut edit_product_barcode = edit_product_barcode.clone();
        let mut edit_product_category = edit_product_category.clone();
        let mut edit_product_price = edit_product_price.clone();
        let mut edit_product_stock = edit_product_stock.clone();
        let mut edit_product_image_preview = edit_product_image_preview.clone();
        let mut edit_product_image_payload = edit_product_image_payload.clone();
        let mut edit_product_image_name = edit_product_image_name.clone();
        let mut edit_product_image_type = edit_product_image_type.clone();
        let mut show_edit_modal = show_edit_modal.clone();
        let mut products_state = products_state.clone();
        let mut delete_msg = delete_msg.clone();
        let mut delete_error = delete_error.clone();
        let pid_for_delete = pid.clone();
        product_rows.push(rsx! {
            tr {
                td { "{pbarcode}" }
                td { "{pname}" }
                td { "{pcategory}" }
                td { "{format_price(pprice)}" }
                td { "{pqty}" }
                td { span { class: "{status_class}", "{status}" } }
                td { div { class: "table-actions",
                    button {
                        class: "btn-small",
                        onclick: move |_| {
                            edit_product_id.set(Some(pid.clone()));
                            edit_product_name.set(pname.clone());
                            edit_product_barcode.set(pbarcode.clone());
                            edit_product_category.set(pcategory.clone());
                            edit_product_price.set(format!("{:.2}", pprice));
                            edit_product_stock.set(format!("{}", pqty));
                            edit_product_image_name.set(pimage_name.clone());
                            edit_product_image_type.set(pimage_type.clone());
                            edit_product_image_payload.set(None);
                            let preview = pimage_name
                                .as_deref()
                                .and_then(|name| image_preview_from_filename(name, &pimage_type, PRODUCTS_IMAGES_DIR));
                            edit_product_image_preview.set(preview);
                            show_edit_modal.set(true);
                        },
                        "✏️ Edit"
                    }
                    button {
                        class: "btn-small btn-danger",
                        onclick: move |_| {
                            let mut updated = products_state.read().clone();
                            updated.retain(|prod| prod.id != pid_for_delete);
                            match save_products_to_json(&updated) {
                                Ok(_) => {
                                    products_state.set(updated);
                                    delete_error.set(None);
                                    delete_msg.set(Some("✅ Product deleted.".to_string()));
                                    spawn(async move {
                                        sleep_ms(2_000).await;
                                        delete_msg.set(None);
                                    });
                                }
                                Err(err) => delete_error.set(Some(err)),
                            }
                        },
                        "🗑️ Delete"
                    }
                } }
            }
        });
    }

    rsx! {
        div { class: "content-card",
            div { class: "products-header",
                h2 { "📦 Manage Products" }
                div { class: "products-header-actions",
                    span { class: "ops-pill warning", "Low stock < {LOW_STOCK_THRESHOLD}" }
                    div { class: "filter-group filter-inline",
                        label { "Search" }
                        input {
                            placeholder: "Name, barcode, category",
                            value: "{product_query}",
                            oninput: move |e| {
                                product_query.set(e.value());
                                product_page.set(1);
                            },
                        }
                    }
                    div { class: "filter-group filter-inline",
                        label { "Sort" }
                        select {
                            value: "{product_sort}",
                            onchange: move |e| product_sort.set(e.value()),
                            option { value: "name", "Name" }
                            option { value: "stock", "Stock" }
                            option { value: "price", "Price" }
                            option { value: "value", "Value" }
                        }
                    }
                    div { class: "filter-group filter-inline",
                        label { "Order" }
                        select {
                            value: "{product_order}",
                            onchange: move |e| product_order.set(e.value()),
                            option { value: "asc", "Asc" }
                            option { value: "desc", "Desc" }
                        }
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_add_modal.set(true),
                        "+ Add Product"
                    }
                }
            }
            
            if edit_save_msg.read().clone() {
                div { class: "message",
                    "✅ Product updated successfully!"
                }
            }
            if let Some(err) = edit_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if add_save_msg.read().clone() {
                div { class: "message",
                    "✅ Product created successfully!"
                }
            }
            if let Some(err) = add_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = delete_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = delete_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            if let Some(msg) = product_export_msg.read().clone() {
                div { class: "message",
                    "{msg}"
                }
            }
            if let Some(err) = product_export_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }

            div { class: "table-container",
                table {
                    thead {
                        tr {
                            th { "Barcode" }
                            th { "Name" }
                            th { "Category" }
                            th { "Price" }
                            th { "Stock" }
                            th { "Status" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if product_rows.is_empty() {
                            tr { td { colspan: "7",
                                div { class: "empty-state", "No products match your filters." }
                            } }
                        } else {
                            { product_rows.into_iter() }
                        }
                    }
                }
            }

            div { class: "ops-summary",
                div { class: "ops-pill", "Page {current_page} / {total_pages}" }
                button {
                    class: "btn-small",
                    onclick: move |_| {
                        match export_products_csv(&products) {
                            Ok(path) => {
                                product_export_error.set(None);
                                product_export_msg.set(Some(format!("✅ Exported to {}", path)));
                            }
                            Err(err) => product_export_error.set(Some(err)),
                        }
                    },
                    "⬇️ Export CSV"
                }
                button {
                    class: "btn-small",
                    disabled: current_page <= 1,
                    onclick: move |_| product_page.set(current_page.saturating_sub(1)),
                    "Prev"
                }
                button {
                    class: "btn-small",
                    disabled: current_page >= total_pages,
                    onclick: move |_| product_page.set(current_page + 1),
                    "Next"
                }
            }
            
            if show_add_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "➕ Add New Product" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_add_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Product Name *" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    placeholder: "e.g., Apple",
                                    value: "{product_name}",
                                    oninput: move |e| product_name.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Barcode *" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    placeholder: "e.g., 123456789012",
                                    value: "{product_barcode}",
                                    oninput: move |e| product_barcode.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    placeholder: "e.g., Fruits",
                                    value: "{product_category}",
                                    oninput: move |e| product_category.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Price ($)" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "💲" }
                                input {
                                    placeholder: "e.g., 1.50",
                                    value: "{product_price}",
                                    oninput: move |e| product_price.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input {
                                    placeholder: "e.g., 100",
                                    value: "{product_stock}",
                                    oninput: move |e| product_stock.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Product Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut product_image_payload = product_image_payload.clone();
                                  let mut product_image_type = product_image_type.clone();
                                  let mut product_image_preview = product_image_preview.clone();
                                  let mut add_error = add_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    product_image_payload.set(Some(payload));
                                                    product_image_type.set(Some(img_type));
                                                    product_image_preview.set(Some(preview));
                                                }
                                                Err(err) => {
                                                    add_error.set(Some(format!("Image load failed: {}", err)));
                                                }
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = product_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_add_product,
                                "✅ Save Product"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_add_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }

            if show_edit_modal.read().clone() {
                div { class: "modal",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h3 { "✏️ Edit Product" }
                            button {
                                class: "modal-close",
                                onclick: move |_| show_edit_modal.set(false),
                                "✕"
                            }
                        }

                        div { class: "form-group",
                            label { "Product Name" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📝" }
                                input {
                                    value: "{edit_product_name}",
                                    oninput: move |e| edit_product_name.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Barcode" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🏷️" }
                                input {
                                    value: "{edit_product_barcode}",
                                    oninput: move |e| edit_product_barcode.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Category" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "🗂️" }
                                input {
                                    value: "{edit_product_category}",
                                    oninput: move |e| edit_product_category.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Price ($)" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "💲" }
                                input {
                                    value: "{edit_product_price}",
                                    oninput: move |e| edit_product_price.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Stock" }
                            div { class: "input-group",
                                span { class: "input-icon-left", "📦" }
                                input {
                                    value: "{edit_product_stock}",
                                    oninput: move |e| edit_product_stock.set(e.value()),
                                }
                            }
                        }

                        div { class: "form-group",
                            label { "Product Image" }
                            div { class: "file-input-wrapper",
                                button { class: "btn btn-secondary", "📷 Choose Image" }
                                { let mut edit_product_image_payload = edit_product_image_payload.clone();
                                  let mut edit_product_image_type = edit_product_image_type.clone();
                                  let mut edit_product_image_preview = edit_product_image_preview.clone();
                                  let mut edit_error = edit_error.clone();
                                  rsx!(
                                    input {
                                        r#type: "file",
                                        accept: "image/*",
                                        onchange: move |e| {
                                            let path = e.value();
                                            if path.is_empty() {
                                                return;
                                            }
                                            match load_image_from_path(&path) {
                                                Ok((payload, img_type, preview)) => {
                                                    edit_product_image_payload.set(Some(payload));
                                                    edit_product_image_type.set(Some(img_type));
                                                    edit_product_image_preview.set(Some(preview));
                                                }
                                                Err(err) => {
                                                    edit_error.set(Some(format!("Image load failed: {}", err)));
                                                }
                                            }
                                        },
                                    }
                                  )
                                }
                            }
                            if let Some(img) = edit_product_image_preview.read().clone() {
                                div { class: "image-preview",
                                    img { src: "{img}" }
                                }
                            }
                        }

                        div { class: "form-actions",
                            button {
                                class: "btn btn-primary",
                                onclick: handle_edit_product,
                                "✅ Save Changes"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(false),
                                "❌ Cancel"
                            }
                        }
                    }
                }
            }

            div { class: "data-ops-grid",
                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Users Snapshot" }
                        span { class: "card-note", "{active_users_s} active · {inactive_users_s} inactive" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total users" }
                            strong { "{users.len()}" }
                        }
                        div { class: "data-stat",
                            span { "Active" }
                            strong { "{active_users_s}" }
                        }
                        div { class: "data-stat",
                            span { "Inactive" }
                            strong { "{inactive_users_s}" }
                        }
                    }
                    div { class: "data-list",
                        { users.iter().take(4).map(|u| {
                            let role_label = match u.role {
                                crate::data::models::user::UserRole::Admin => "Admin",
                                crate::data::models::user::UserRole::Staff => "Staff",
                            };
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{u.username}" }
                                        span { "{u.email}" }
                                    }
                                    span { class: "data-chip", "{role_label}" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Products Snapshot" }
                        span { class: "card-note", "{total_products_s} items" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total stock" }
                            strong { "{total_stock_s}" }
                        }
                        div { class: "data-stat",
                            span { "Low stock" }
                            strong { "{low_stock_count}" }
                        }
                        div { class: "data-stat",
                            span { "Out of stock" }
                            strong { "{out_of_stock}" }
                        }
                    }
                    div { class: "data-list",
                        { products_sorted.iter().take(4).map(|p| {
                            let value = format_price(p.price);
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "{p.name}" }
                                        span { "{p.category} · {value}" }
                                    }
                                    span { class: "data-chip", "{p.quantity} in stock" }
                                }
                            )
                        }) }
                    }
                }

                div { class: "content-card data-card",
                    div { class: "card-header-row",
                        h3 { "Sales Snapshot" }
                        span { class: "card-note", "{total_transactions_s}" }
                    }
                    div { class: "data-stats",
                        div { class: "data-stat",
                            span { "Total revenue" }
                            strong { "{total_revenue_s}" }
                        }
                        div { class: "data-stat",
                            span { "Avg ticket" }
                            strong { "{average_ticket_s}" }
                        }
                        div { class: "data-stat",
                            span { "Last update" }
                            strong { "{last_updated}" }
                        }
                    }
                    div { class: "data-list",
                        { sales.iter().rev().take(4).map(|s| {
                            let amount = format_price(s.total);
                            let when = format_datetime(&s.created_at);
                            rsx!(
                                div { class: "data-row",
                                    div { class: "data-main",
                                        strong { "Sale {s.id}" }
                                        span { "{when}" }
                                    }
                                    span { class: "data-chip", "{amount}" }
                                }
                            )
                        }) }
                    }
                }
            }
        }
    }
}
