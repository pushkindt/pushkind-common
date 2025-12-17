use actix_identity::Identity;
use actix_web::http::header;
use actix_web::{HttpResponse, Responder, get, post, web};
use actix_web_flash_messages::{IncomingFlashMessages, Level};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::domain::auth::AuthenticatedUser;
use crate::models::config::CommonServerConfig;
use crate::services::errors::{ServiceError, ServiceResult};

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.and_then(|s| {
        let trimmed = s.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }))
}

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
pub fn ensure_role(user: &AuthenticatedUser, role: &str) -> ServiceResult<()> {
    if check_role(role, &user.roles) {
        Ok(())
    } else {
        Err(ServiceError::Unauthorized)
    }
}

/// Render a Tera template with the provided context and return an HTTP response.
///
/// If template rendering fails, logs the error and returns an empty response body.
pub fn render_template(tera: &Tera, template: &str, context: &Context) -> HttpResponse {
    HttpResponse::Ok().body(tera.render(template, context).unwrap_or_else(|e| {
        log::error!("Failed to render template '{template}': {e}");
        String::new()
    }))
}

/// Create a base template context with common variables.
///
/// Includes flash message alerts, current user, current page, and home URL.
pub fn base_context(
    flash_messages: &IncomingFlashMessages,
    user: &AuthenticatedUser,
    current_page: &str,
    home_url: &str,
) -> Context {
    let alerts = flash_messages
        .iter()
        .map(|f| (f.content(), alert_level_to_str(&f.level())))
        .collect::<Vec<_>>();

    let mut context = Context::new();
    context.insert("alerts", &alerts);
    context.insert("current_user", user);
    context.insert("current_page", current_page);
    context.insert("home_url", home_url);
    context
}

#[post("/logout")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    redirect("/")
}

#[get("/na")]
pub async fn not_assigned(
    user: AuthenticatedUser,
    flash_messages: IncomingFlashMessages,
    server_config: web::Data<CommonServerConfig>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let context = base_context(
        &flash_messages,
        &user,
        "index",
        &server_config.auth_service_url,
    );

    render_template(&tera, "main/not_assigned.html", &context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use actix_web::http::header;
    use actix_web::http::header::HeaderValue;
    use actix_web_flash_messages::Level;
    use tera::{Context, Tera};

    fn sample_user(roles: Vec<&str>) -> AuthenticatedUser {
        AuthenticatedUser {
            sub: "1".to_string(),
            email: "test@example.com".to_string(),
            hub_id: 1,
            name: "Test".to_string(),
            roles: roles.into_iter().map(|r| r.to_string()).collect(),
            exp: 0,
        }
    }

    #[actix_web::test]
    async fn check_role_detects_role() {
        assert!(check_role("admin", ["user", "admin"]));
        assert!(!check_role("admin", ["user", "manager"]));
    }

    #[actix_web::test]
    async fn redirect_sets_location_header() {
        let resp = redirect("/target");
        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(
            resp.headers().get(header::LOCATION),
            Some(&HeaderValue::from_static("/target"))
        );
    }

    #[actix_web::test]
    async fn test_alert_level_to_str_mappings() {
        assert_eq!(alert_level_to_str(&Level::Error), "danger");
        assert_eq!(alert_level_to_str(&Level::Warning), "warning");
        assert_eq!(alert_level_to_str(&Level::Success), "success");
        assert_eq!(alert_level_to_str(&Level::Info), "info");
        assert_eq!(alert_level_to_str(&Level::Debug), "info");
    }

    #[actix_web::test]
    async fn ensure_role_allows_matching_role() {
        let user = sample_user(vec!["admin"]);
        assert!(ensure_role(&user, "admin").is_ok());
    }

    #[actix_web::test]
    async fn ensure_role_denies_missing_role() {
        let user = sample_user(vec!["user"]);
        assert!(matches!(
            ensure_role(&user, "admin"),
            Err(ServiceError::Unauthorized)
        ));
    }

    #[actix_web::test]
    async fn render_template_returns_ok_with_rendered_body_on_success() {
        let mut tera = Tera::default();
        if let Err(err) = tera.add_raw_template("hello.txt", "Hi {{ name }}") {
            panic!("failed to add template: {err}");
        }

        let mut ctx = Context::new();
        ctx.insert("name", "Slava");

        let resp = render_template(&tera, "hello.txt", &ctx);

        assert_eq!(resp.status(), StatusCode::OK);

        let body = match to_bytes(resp.into_body()).await {
            Ok(body) => body,
            Err(err) => panic!("failed to read response body: {err}"),
        };
        assert_eq!(&body[..], b"Hi Slava");
    }

    #[actix_web::test]
    async fn render_template_returns_ok_with_empty_body_on_failure() {
        // No templates registered -> rendering will fail.
        let tera = Tera::default();
        let ctx = Context::new();

        let resp = render_template(&tera, "missing.txt", &ctx);

        assert_eq!(resp.status(), StatusCode::OK);

        let body = match to_bytes(resp.into_body()).await {
            Ok(body) => body,
            Err(err) => panic!("failed to read response body: {err}"),
        };
        assert!(body.is_empty(), "body should be empty on render error");
    }
}
