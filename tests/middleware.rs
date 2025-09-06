#![cfg(feature = "actix")]
use actix_web::{
    App, HttpResponse,
    http::{StatusCode, header},
    test, web,
};

use pushkind_common::{middleware::RedirectUnauthorized, models::config::CommonServerConfig};

#[actix_web::test]
async fn redirects_unauthorized_to_signin() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "http://auth.test.me/".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "http://auth.test.me/?next=http%3A%2F%2Flocalhost%3A8080%2F"
    );
}

#[actix_web::test]
async fn redirects_unauthorized_to_relative_signin() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "/auth/signin".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "/auth/signin?next=http%3A%2F%2Flocalhost%3A8080%2F"
    );
}

#[actix_web::test]
async fn redirects_unauthorized_to_relative_signin_with_fragment() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "/auth/signin#step2".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "/auth/signin?next=http%3A%2F%2Flocalhost%3A8080%2F#step2",
    );
}

#[actix_web::test]
async fn does_not_duplicate_next_param_for_absolute_url() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "http://auth.test.me/?next=custom".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "http://auth.test.me/?next=custom",
    );
}

#[actix_web::test]
async fn does_not_duplicate_next_param_for_relative_url() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "/auth/signin?next=custom".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "/auth/signin?next=custom",
    );
}

#[actix_web::test]
async fn success_response_passes_through() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "http://auth.test.me/".to_string(),
    };
    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Ok().finish() })),
    )
    .await;

    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn uses_inner_next_value_for_absolute_auth_url() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "http://auth.test.me/".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    // Incoming URL contains its own `next` parameter
    let req = test::TestRequest::default()
        .uri("/path?x=1&next=https%3A%2F%2Fexample.com%2Ftarget%3Fa%3D1%26b%3D2&y=2")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "http://auth.test.me/?next=https%3A%2F%2Fexample.com%2Ftarget%3Fa%3D1%26b%3D2",
    );
}

#[actix_web::test]
async fn uses_inner_next_value_for_relative_auth_url() {
    let server_config = CommonServerConfig {
        secret: "secret".to_string(),
        auth_service_url: "/auth/signin".to_string(),
    };

    let app = test::init_service(
        App::new()
            .wrap(RedirectUnauthorized)
            .app_data(web::Data::new(server_config.clone()))
            .default_service(web::to(|| async { HttpResponse::Unauthorized().finish() })),
    )
    .await;

    // Incoming URL contains its own `next` parameter
    let req = test::TestRequest::default()
        .uri("/path?next=https%3A%2F%2Fexample.com%2Fwelcome")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    assert_eq!(
        resp.headers().get(header::LOCATION).unwrap(),
        "/auth/signin?next=https%3A%2F%2Fexample.com%2Fwelcome",
    );
}
