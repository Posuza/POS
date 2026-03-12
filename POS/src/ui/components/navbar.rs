/// Navbar component

use dioxus::prelude::*;
use crate::data::models::user::User;
use crate::services::image_service::ImageService;
use crate::config::constants::PROFILES_IMAGES_DIR;

#[component]
pub fn Navbar(
    title: String,
    user: Option<User>,
    on_login: EventHandler<()>,
    on_logout: EventHandler<()>,
) -> Element {
    // Build optional image src if profile image and type are present.
    // Support three cases:
    // 1) `profile_image` already contains a data URL (starts with "data:") -> use as-is
    // 2) `profile_image` contains a stored filename -> load file from profiles folder and convert to data URL
    // 3) absent -> None (will fall back to initials)
    let image_src = user.as_ref().and_then(|u| {
        if let (Some(img), Some(img_t)) = (u.profile_image.as_ref(), u.profile_image_type.as_ref()) {
            if img.starts_with("data:") {
                Some(img.clone())
            } else {
                match ImageService::get_image_data_url(img, img_t, PROFILES_IMAGES_DIR) {
                    Ok(data_url) => Some(data_url),
                    Err(_) => None,
                }
            }
        } else {
            None
        }
    });

    // Compute display initials for fallback avatar when there's no profile image
    let initials = user.as_ref().map(|u| {
        let mut letters: String = u.username
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().next().unwrap_or('?'))
            .take(2)
            .collect();
        if letters.is_empty() {
            letters = u.username.chars().take(2).collect();
        }
        letters.to_uppercase()
    });

    // Try to load the app logo from the repo images folder as a data URL so
    // the navbar can render it reliably in the runtime environment.
    let logo_src = match ImageService::get_image_data_url(
        "image1.jpeg",
        "image/jpeg",
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/images/logo"),
    ) {
        Ok(s) => Some(s),
        Err(_) => None,
    };

    rsx! {
        nav { class: "navbar",
            // Left-most icon (logo image)
            div { class: "navbar-icon-left",
                if let Some(src) = &logo_src {
                    img { class: "navbar-icon-img", src: "{src}", alt: "logo" }
                } else {
                    span { class: "navbar-icon-placeholder", "Pos_image" }
                }
            }

            // Brand centered in the navbar
            span { class: "navbar-brand", {title} }

            // Profile/login area placed at the right-most edge of the viewport
            div { class: "navbar-right-outer",
                if user.is_none() {
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| on_login.call(()),
                        "Login"
                    }
                } else if let Some(u) = user {
                    if let Some(src) = &image_src {
                        img { class: "navbar-user-img", src: "{src}" }
                    } else if let Some(i) = &initials {
                        div { class: "navbar-avatar-fallback", "{i}" }
                    } else {
                        div { class: "navbar-avatar-fallback", "U" }
                    }
                    span { class: "navbar-user", {u.username} }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| on_logout.call(()),
                        "Logout"
                    }
                }
            }
        }
    }
}
