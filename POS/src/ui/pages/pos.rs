use crate::config::constants::APP_NAME;
use crate::config::constants::PRODUCTS_IMAGES_DIR;
use crate::data::json_store::get_store;
use crate::data::models::user::User;
use crate::services::image_service::ImageService;
use crate::ui::components::Navbar;
use crate::utils::formatters::format_price;
use dioxus::prelude::*;

#[component]
pub fn POSPage(user: User, on_logout: EventHandler<()>) -> Element {
    let mut barcode_input = use_signal(|| String::new());
    // cart entries: (name, unit_price, quantity)
    let mut cart = use_signal(|| Vec::<(String, f32, i32)>::new());
    let mut total = use_signal(|| 0.0f32);
    let mut message = use_signal(|| None::<String>);
    let mut current_product_image = use_signal(|| None::<String>);
    let mut show_checkout_modal = use_signal(|| false);
    let payment_method = use_signal(|| "Cash".to_string());
    let mut show_cart_mobile = use_signal(|| false);
    // snapshot for rendering to avoid simultaneous immutable/mutable borrows in RSX
    let cart_open = *show_cart_mobile.read();
    let store = get_store();

    let handle_scan = move |_: Event<dioxus::events::MouseData>| {
        let barcode = {
            let b = barcode_input.read();
            if b.is_empty() {
                return;
            }
            b.clone()
        };

        let product = store.find_product_by_barcode(&barcode);

        if let Some(product) = product {
            let image_url = match (&product.product_image, &product.product_image_type) {
                (Some(img), Some(img_type)) if img.starts_with("data:") => Some(img.clone()),
                (Some(img), Some(img_type)) => {
                    ImageService::get_image_data_url(img, img_type, PRODUCTS_IMAGES_DIR).ok()
                }
                _ => None,
            };
            // add or increment item in cart
            let mut c = cart.write();
            if let Some(pos) = c.iter().position(|it| it.0 == product.name) {
                c[pos].2 += 1;
            } else {
                c.push((product.name.clone(), product.price, 1));
            }
            current_product_image.set(image_url);

            let new_total: f32 = *total.read() + product.price;
            total.set(new_total);
            message.set(Some(format!("✅ {} added!", product.name)));

            // Clear message after 2 seconds
            spawn(async move {
                gloo_timers::future::sleep(std::time::Duration::from_secs(2)).await;
                message.set(None);
            });
        } else {
            message.set(Some("❌ Product not found".to_string()));
        }

        barcode_input.set(String::new());
    };

    // Removed separate `handle_remove_item` closure to avoid borrow/mutability issues;
    // removal is performed inline where needed.

    let handle_clear_cart = move |_: Event<dioxus::events::MouseData>| {
        cart.write().clear();
        total.set(0.0);
        message.set(Some("🗑️ Cart cleared".to_string()));
    };

    let handle_checkout = move |_: Event<dioxus::events::MouseData>| {
        if cart.read().is_empty() {
            message.set(Some("❌ Cart is empty".to_string()));
            return;
        }
        show_checkout_modal.set(true);
    };

    let _handle_complete_sale = move |_: Event<dioxus::events::MouseData>| {
        // In real app, save transaction to database (scans table)
        let t: f32 = *total.read();
        let pm = payment_method.read().clone();
        message.set(Some(format!(
            "✅ Sale complete! Total: ${:.2} via {}",
            t, pm
        )));
        cart.write().clear();
        total.set(0.0);
        show_checkout_modal.set(false);

        spawn(async move {
            gloo_timers::future::sleep(std::time::Duration::from_secs(3)).await;
            message.set(None);
        });
    };

    rsx! {
        div { class: "pos-page",
            Navbar {
                title: APP_NAME.to_string(),
                user: Some(user.clone()),
                on_login: move |_| (),
                on_logout: on_logout.clone(),
            }

            div { class: "pos-container",
                div { class: "pos-grid",
                    // Left: product/scan
                    div { class: "pos-left",
                        div { class: "product-display",
                            if let Some(src) = current_product_image.read().clone() {
                                img { class: "product-img", src: "{src}" }
                            }
                        }
                        div { class: "scan-area",
                            div { class: "input-group",
                                span { class: "input-icon-left", "🔍" }
                                input {
                                    class: "form-input",
                                    placeholder: "Scan or enter barcode",
                                    value: "{barcode_input}",
                                    oninput: move |e| barcode_input.set(e.value()),
                                }
                            }
                            button { class: "btn btn-primary", onclick: handle_scan, "Scan" }
                        }

                        div { class: "product-list",
                            h4 { "Quick Add" }
                            ul {
                                { store.products.iter().take(4).map(|p| rsx!{
                                    li { key: "{p.barcode}", class: "product-row",
                                        div { class: "product-row-left",
                                            span { class: "product-name", "{p.name}" }
                                            span { class: "product-cat", "{p.category}" }
                                        }
                                        div { class: "product-row-right",
                                            span { class: "product-price", "{format_price(p.price)}" }
                                            button { class: "btn btn-secondary", onclick: move |_| {
                                                let mut c = cart.write();
                                                if let Some(pos) = c.iter().position(|it| it.0 == p.name) {
                                                    c[pos].2 += 1;
                                                } else {
                                                    c.push((p.name.clone(), p.price, 1));
                                                }
                                                let current_total: f32 = *total.read();
                                                total.set(current_total + p.price);
                                            }, "Add" }
                                        }
                                    }
                                }) }
                            }
                        }
                    }

                    // Right: cart
                    div { class: if cart_open { "pos-right open" } else { "pos-right" },
                        h3 { "Cart" }
                        // mobile close control (visible via CSS on small screens)
                        button { class: "cart-close-mobile", onclick: move |_| show_cart_mobile.set(false), "×" }
                        div { class: "cart-list",
                            ul {
                                { cart.read().iter().enumerate().map(|(i, item)| rsx!{
                                    li { key: "cart-{i}", class: "cart-row",
                                        div { class: "cart-row-left",
                                            span { "{item.0}" }
                                        }

                                        div { class: "cart-row-center",
                                            div { class: "qty-controls",
                                                button { class: "btn-small btn-secondary", onclick: move |_| {
                                                    let mut c = cart.write();
                                                    if let Some(it) = c.get_mut(i) {
                                                        it.2 += 1;
                                                        let current_total: f32 = *total.read();
                                                        total.set(current_total + it.1);
                                                    }
                                                }, "+" }

                                                span { class: "qty-num", "{item.2}" }

                                                button { class: "btn-small btn-danger", onclick: move |_| {
                                                    let mut c = cart.write();
                                                    if i < c.len() {
                                                        // must re-check length because other closures may have mutated
                                                        if let Some(price) = c.get(i).map(|it| it.1) {
                                                            if c[i].2 > 1 {
                                                                c[i].2 -= 1;
                                                                let current_total: f32 = *total.read();
                                                                total.set(current_total - price);
                                                            } else {
                                                                c.remove(i);
                                                                let current_total: f32 = *total.read();
                                                                total.set((current_total - price).max(0.0));
                                                            }
                                                        }
                                                    }
                                                }, "-" }
                                            }
                                        }

                                        div { class: "cart-row-right",
                                            span { class: "cart-price", "{format_price(item.1 * item.2 as f32)}" }
                                        }
                                    }
                                }) }
                            }
                        }

                        div { class: "cart-summary",
                            div { class: "cart-total", "Total: {format_price(*total.read())}" }
                            div { class: "cart-actions",
                                button { class: "btn btn-secondary", onclick: handle_clear_cart, "Clear" }
                                button { class: "btn btn-primary", onclick: handle_checkout, "Checkout" }
                            }
                        }
                    }
                }
            }

            // Mobile floating cart button (FAB) — only show when cart is closed
            if !cart_open {
                button { class: "cart-fab", onclick: move |_| {
                    let current = *show_cart_mobile.read();
                    show_cart_mobile.set(!current);
                }, "🛒 {cart.read().len()}" }
            }
        }
    }
}
