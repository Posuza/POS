use super::prelude::*;

#[component]
pub(crate) fn CreateProductTab() -> Element {
    let store = get_store_fresh();
    let categories = extra_list("categories.json");
    let sub_categories = extra_list("sub_categories.json");
    let brands = extra_list("brands.json");
    let units = extra_list("units.json");
    let warranties = extra_list("warranties.json");
    let warehouses = extra_list("warehouses.json");
    let stores = extra_list("stores.json");
    let mut products_state = use_signal(|| store.products.clone());
    let mut save_msg = use_signal(|| false);
    let mut save_error = use_signal(|| None::<String>);
    let mut store_name = use_signal(|| String::new());
    let mut warehouse_name = use_signal(|| String::new());
    let mut product_name = use_signal(|| String::new());
    let mut sku = use_signal(|| String::new());
    let mut category = use_signal(|| String::new());
    let mut sub_category = use_signal(|| String::new());
    let mut brand = use_signal(|| String::new());
    let mut unit = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut quantity = use_signal(|| String::new());
    let mut price = use_signal(|| String::new());
    let mut tax_type = use_signal(|| String::new());
    let mut discount = use_signal(|| String::new());
    let mut alert_qty = use_signal(|| String::new());
    let mut warranty = use_signal(|| String::new());
    let mut manufacturer = use_signal(|| String::new());
    let mut manufactured_date = use_signal(|| String::new());
    let mut expiry_date = use_signal(|| String::new());
    let mut image_payload = use_signal(|| None::<String>);
    let mut image_type = use_signal(|| None::<String>);
    let mut image_preview = use_signal(|| None::<String>);

    let handle_save = {
        let mut products_state = products_state.clone();
        let mut save_msg = save_msg.clone();
        let mut save_error = save_error.clone();
        let mut product_name = product_name.clone();
        let mut sku = sku.clone();
        let mut category = category.clone();
        let mut description = description.clone();
        let mut quantity = quantity.clone();
        let mut price = price.clone();
        let mut store_name = store_name.clone();
        let mut warehouse_name = warehouse_name.clone();
        let mut sub_category = sub_category.clone();
        let mut brand = brand.clone();
        let mut unit = unit.clone();
        let mut tax_type = tax_type.clone();
        let mut discount = discount.clone();
        let mut alert_qty = alert_qty.clone();
        let mut warranty = warranty.clone();
        let mut manufacturer = manufacturer.clone();
        let mut manufactured_date = manufactured_date.clone();
        let mut expiry_date = expiry_date.clone();
        let mut image_payload = image_payload.clone();
        let mut image_type = image_type.clone();
        let mut image_preview = image_preview.clone();
        move |_| {
            if product_name.read().is_empty() || sku.read().is_empty() {
                save_error.set(Some("Product name and SKU are required.".to_string()));
                return;
            }
            let price_val = match price.read().parse::<f32>() {
                Ok(v) if v >= 0.01 => v,
                Err(_) => {
                    save_error.set(Some("Price must be a valid number.".to_string()));
                    return;
                }
                _ => {
                    save_error.set(Some("Price must be at least 0.01.".to_string()));
                    return;
                }
            };
            let qty_val = match quantity.read().parse::<i32>() {
                Ok(v) if v >= 0 => v,
                Err(_) => {
                    save_error.set(Some("Quantity must be a valid number.".to_string()));
                    return;
                }
                _ => {
                    save_error.set(Some("Quantity must be a non-negative number.".to_string()));
                    return;
                }
            };
            let mut updated = products_state.read().clone();
            if updated.iter().any(|p| p.barcode == *sku.read()) {
                save_error.set(Some("A product with this SKU/Barcode already exists.".to_string()));
                return;
            }
            let mut image_filename: Option<String> = None;
            let mut image_type_saved: Option<String> = None;
            if let (Some(payload), Some(img_type)) = (image_payload.read().clone(), image_type.read().clone()) {
                match ImageService::save_product_image(&payload, &img_type) {
                    Ok(filename) => {
                        image_filename = Some(filename);
                        image_type_saved = Some(img_type);
                    }
                    Err(err) => {
                        save_error.set(Some(format!("Image upload failed: {}", err)));
                        return;
                    }
                }
            }
            let now = now_iso();
            let product = Product {
                id: new_id("prod"),
                barcode: sku.read().clone(),
                name: product_name.read().clone(),
                description: if description.read().is_empty() { None } else { Some(description.read().clone()) },
                price: price_val,
                quantity: qty_val,
                category: if category.read().is_empty() { "Uncategorized".to_string() } else { category.read().clone() },
                product_image: image_filename,
                product_image_type: image_type_saved,
                created_at: now.clone(),
                updated_at: now,
            };
            updated.push(product);
            match save_products_to_json(&updated) {
                Ok(_) => {
                    products_state.set(updated);
                    save_error.set(None);
                    save_msg.set(true);
                    product_name.set(String::new());
                    sku.set(String::new());
                    category.set(String::new());
                    sub_category.set(String::new());
                    brand.set(String::new());
                    unit.set(String::new());
                    description.set(String::new());
                    quantity.set(String::new());
                    price.set(String::new());
                    tax_type.set(String::new());
                    discount.set(String::new());
                    alert_qty.set(String::new());
                    warranty.set(String::new());
                    manufacturer.set(String::new());
                    manufactured_date.set(String::new());
                    expiry_date.set(String::new());
                    store_name.set(String::new());
                    warehouse_name.set(String::new());
                    image_payload.set(None);
                    image_type.set(None);
                    image_preview.set(None);
                    spawn(async move {
                        sleep_ms(2_000).await;
                        save_msg.set(false);
                    });
                }
                Err(err) => save_error.set(Some(err)),
            }
        }
    };

    rsx! {
        div { class: "admin-form-page",
            if save_msg.read().clone() {
                div { class: "message",
                    "✅ Product created successfully!"
                }
            }
            if let Some(err) = save_error.read().clone() {
                div { class: "message error",
                    "{err}"
                }
            }
            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Product Information" }
                    span { class: "card-subtitle", "Fill in the core details for a new product." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Store" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{store_name}",
                                onchange: move |e| store_name.set(e.value()),
                                option { value: "", "Select" }
                                { stores.iter().map(|s| {
                                    let name = pick_first(s, &["store", "name", "title"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Warehouse" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{warehouse_name}",
                                onchange: move |e| warehouse_name.set(e.value()),
                                option { value: "", "Select" }
                                { warehouses.iter().map(|w| {
                                    let name = pick_first(w, &["warehouse", "name", "title"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Product Name" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            input {
                                r#type: "text",
                                placeholder: "Apple iPhone 15",
                                value: "{product_name}",
                                oninput: move |e| product_name.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "SKU" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏷️" }
                            input {
                                r#type: "text",
                                placeholder: "PT001",
                                value: "{sku}",
                                oninput: move |e| sku.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Category" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{category}",
                                onchange: move |e| category.set(e.value()),
                                option { value: "", "Select" }
                                { categories.iter().map(|c| {
                                    let name = pick_first(c, &["name", "category"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Sub Category" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{sub_category}",
                                onchange: move |e| sub_category.set(e.value()),
                                option { value: "", "Select" }
                                { sub_categories.iter().map(|c| {
                                    let name = pick_first(c, &["sub_category", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Brand" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{brand}",
                                onchange: move |e| brand.set(e.value()),
                                option { value: "", "Select" }
                                { brands.iter().map(|b| {
                                    let name = pick_first(b, &["brand", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Unit" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{unit}",
                                onchange: move |e| unit.set(e.value()),
                                option { value: "", "Select" }
                                { units.iter().map(|u| {
                                    let name = pick_first(u, &["unit", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field span-2",
                        label { "Description" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "📝" }
                            textarea {
                                rows: "4",
                                placeholder: "Short product description...",
                                value: "{description}",
                                oninput: move |e| description.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Pricing & Stocks" }
                    span { class: "card-subtitle", "Set pricing, taxes, and inventory alerts." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Quantity" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🔢" }
                            input {
                                r#type: "number",
                                placeholder: "100",
                                value: "{quantity}",
                                oninput: move |e| quantity.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Price" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "💲" }
                            input {
                                r#type: "number",
                                placeholder: "$120.00",
                                value: "{price}",
                                oninput: move |e| price.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Tax Type" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{tax_type}",
                                onchange: move |e| tax_type.set(e.value()),
                                option { value: "", "Select" }
                                option { value: "inclusive", "Inclusive" }
                                option { value: "exclusive", "Exclusive" }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Discount" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "％" }
                            input {
                                r#type: "text",
                                placeholder: "5%",
                                value: "{discount}",
                                oninput: move |e| discount.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Alert Quantity" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "⚠️" }
                            input {
                                r#type: "number",
                                placeholder: "10",
                                value: "{alert_qty}",
                                oninput: move |e| alert_qty.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Images" }
                    span { class: "card-subtitle", "Upload product imagery and gallery assets." }
                }
                div { class: "admin-card-body",
                    div { class: "upload-drop",
                        span { "Drag & drop or click to upload images" }
                        { let mut image_payload = image_payload.clone();
                          let mut image_type = image_type.clone();
                          let mut image_preview = image_preview.clone();
                          let mut save_error = save_error.clone();
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
                                            image_payload.set(Some(payload));
                                            image_type.set(Some(img_type));
                                            image_preview.set(Some(preview));
                                        }
                                        Err(err) => {
                                            save_error.set(Some(format!("Image load failed: {}", err)));
                                        }
                                    }
                                },
                            }
                          )
                        }
                    }
                    if let Some(img) = image_preview.read().clone() {
                        div { class: "image-preview",
                            img { src: "{img}" }
                        }
                    }
                }
            }

            div { class: "admin-card",
                div { class: "admin-card-header",
                    h3 { "Custom Fields" }
                    span { class: "card-subtitle", "Warranty, manufacturer, and expiry data." }
                }
                div { class: "admin-card-body form-grid",
                    div { class: "form-field",
                        label { "Warranty" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "▾" }
                            select {
                                value: "{warranty}",
                                onchange: move |e| warranty.set(e.value()),
                                option { value: "", "Select" }
                                { warranties.iter().map(|w| {
                                    let name = pick_first(w, &["warranty", "name"]);
                                    rsx!( option { value: "{name}", "{name}" } )
                                }) }
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Manufacturer" }
                        div { class: "input-group",
                            span { class: "input-icon-left", "🏭" }
                            input {
                                r#type: "text",
                                placeholder: "Acme Corp",
                                value: "{manufacturer}",
                                oninput: move |e| manufacturer.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Manufactured Date" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "📅" }
                            input {
                                r#type: "date",
                                value: "{manufactured_date}",
                                oninput: move |e| manufactured_date.set(e.value()),
                            }
                        }
                    }
                    div { class: "form-field",
                        label { "Expiry Date" }
                        div { class: "input-group input-right",
                            span { class: "input-icon-right", "📅" }
                            input {
                                r#type: "date",
                                value: "{expiry_date}",
                                oninput: move |e| expiry_date.set(e.value()),
                            }
                        }
                    }
                }
            }

            div { class: "form-actions",
                button { class: "btn-secondary", "Cancel" }
                button {
                    class: "btn-primary",
                    onclick: handle_save,
                    "Add Product"
                }
            }
        }
    }
}
