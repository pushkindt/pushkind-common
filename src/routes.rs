use actix_web::HttpResponse;
use actix_web::http::header;
use actix_web_flash_messages::{FlashMessage, Level};

use crate::models::auth::AuthenticatedUser;

pub const DEFAULT_ITEMS_PER_PAGE: usize = 20;

pub fn alert_level_to_str(level: &Level) -> &'static str {
    match level {
        Level::Error => "danger",
        Level::Warning => "warning",
        Level::Success => "success",
        _ => "info",
    }
}

pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((header::LOCATION, location))
        .finish()
}

pub fn check_role<I, S>(role: &str, roles: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    roles.into_iter().any(|r| r.as_ref() == role)
}

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
    use actix_web::{test, App, http::header, web, HttpResponse};
    use actix_web::cookie::Key;
    use actix_web_flash_messages::storage::CookieMessageStore;
    use actix_web_flash_messages::storage::FlashMessageStore;
    use actix_web_flash_messages::FlashMessagesFramework;

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

    #[test]
    fn ensure_role_allows_matching_role() {
        let user = sample_user(vec!["admin"]);
        let result = ensure_role(&user, "admin", Some("/"));
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn ensure_role_denies_missing_role() {
        async fn handler(user: AuthenticatedUser) -> HttpResponse {
            match ensure_role(&user, "admin", Some("/login")) {
                Ok(()) => HttpResponse::Ok().finish(),
                Err(resp) => resp,
            }
        }

        let key = Key::generate();
        let framework_store = CookieMessageStore::builder(key.clone()).build();
        let framework = FlashMessagesFramework::builder(framework_store).build();

        let app = test::init_service(
            App::new()
                .wrap(framework)
                .default_service(web::to(move || handler(sample_user(vec!["user"]))))
        )
        .await;

        let decode_store = CookieMessageStore::builder(key).build();

        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::SEE_OTHER);
        assert_eq!(resp.headers().get(header::LOCATION).unwrap(), "/login");

        let flash_cookie = resp
            .response()
            .cookies()
            .find(|c| c.name() == "_flash")
            .expect("flash cookie not set");

        assert!(!flash_cookie.value().is_empty());

        let req_with_cookie = test::TestRequest::default()
            .cookie(flash_cookie.clone())
            .to_http_request();

        let messages = decode_store.load(&req_with_cookie).unwrap();
        assert_eq!(messages.len(), 1);
        let message = &messages[0];
        assert_eq!(message.content(), "Недостаточно прав.");
        assert_eq!(message.level(), Level::Error);
    }
}
