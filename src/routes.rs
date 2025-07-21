use actix_web::HttpResponse;
use actix_web::http::header;
use actix_web_flash_messages::{FlashMessage, Level};

use crate::models::auth::AuthenticatedUser;

/// Default number of list items shown when a page size is not specified.
/// This constant is used by pagination helpers throughout the crate.
pub const DEFAULT_ITEMS_PER_PAGE: usize = 20;

/// Convert a [`FlashMessage`] [`Level`] to a CSS class string used by the
/// templates. Unknown levels default to `info`.
pub fn alert_level_to_str(level: &Level) -> &'static str {
    match level {
        Level::Error => "danger",
        Level::Warning => "warning",
        Level::Success => "success",
        _ => "info",
    }
}

/// Create a `303 See Other` [`HttpResponse`] redirecting to the provided URL.
pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, location))
        .finish()
}

/// Check that a collection of roles contains the specified role.
///
/// The collection can be any iterator over items that are referenceable as
/// `str`.
pub fn check_role<I, S>(role: &str, roles: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    roles.into_iter().any(|r| r.as_ref() == role)
}

/// Ensure that the authenticated user has the required role.
///
/// If the role is missing a flash error message is queued and the caller
/// receives an `Err` containing a redirect response to either the provided URL
/// or `"/"`.
pub fn ensure_role(
    user: &AuthenticatedUser,
    role: &str,
    redirect_url: Option<&str>,
) -> Result<(), HttpResponse> {
    if check_role(role, &user.roles) {
        Ok(())
    } else {
        FlashMessage::error("Недостаточно прав.").send();
        Err(redirect(redirect_url.unwrap_or("/")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web_flash_messages::Level;

    #[test]
    fn check_role_detects_role() {
        assert!(check_role("admin", &["user", "admin"]));
        assert!(!check_role("admin", &["user", "manager"]));
    }

    #[test]
    fn redirect_sets_location_header() {
        let resp = redirect("/target");
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(header::LOCATION).unwrap(), "/target");
    }

    #[test]
    fn test_alert_level_to_str_mappings() {
        assert_eq!(alert_level_to_str(&Level::Error), "danger");
        assert_eq!(alert_level_to_str(&Level::Warning), "warning");
        assert_eq!(alert_level_to_str(&Level::Success), "success");
        assert_eq!(alert_level_to_str(&Level::Info), "info");
        assert_eq!(alert_level_to_str(&Level::Debug), "info");
    }
}
