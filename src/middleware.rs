//! Middleware for redirecting unauthorized requests to an external
//! authentication service.
//!
//! The service URL is provided via [`CommonServerConfig`]. When the wrapped
//! service responds with `401 Unauthorized`, a `303 See Other` response is
//! returned pointing to the configured authentication service.

use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web,
};
use futures_util::future::LocalBoxFuture;
use std::future::{Ready, ready};
use url::{Url, form_urlencoded};

use crate::models::config::CommonServerConfig;

/// Middleware factory used to redirect unauthorized requests to the
/// authentication service defined in [`CommonServerConfig`].
///
/// Attach this with `.wrap()` around services that should redirect users when
/// a `401 Unauthorized` response is encountered.
pub struct RedirectUnauthorized;

/// Creates [`RedirectUnauthorizedMiddleware`] without any asynchronous
/// initialization by simply storing the provided service.
impl<S, B> Transform<S, ServiceRequest> for RedirectUnauthorized
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = RedirectUnauthorizedMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectUnauthorizedMiddleware { service }))
    }
}

/// Service produced by [`RedirectUnauthorized`] that wraps another service
/// and handles unauthorized responses.
pub struct RedirectUnauthorizedMiddleware<S> {
    service: S,
}

fn build_redirect_url(auth_service_url: &str, incoming_url: &str) -> Result<String, Error> {
    // Determine what to pass as the `next` value:
    // if the incoming URL already contains a `next` query parameter, use its value;
    // otherwise, use the entire incoming URL.
    let next_value = match Url::parse(incoming_url) {
        Ok(url) => url
            .query_pairs()
            .find(|(k, _)| k == "next")
            .map(|(_, v)| v.into_owned())
            .unwrap_or_else(|| incoming_url.to_string()),
        Err(_) => {
            // Fallback for cases where `incoming_url` can't be parsed as absolute URL.
            // Try to manually parse query params.
            let base = incoming_url
                .split_once('#')
                .map(|(b, _)| b)
                .unwrap_or(incoming_url);
            let maybe_next = base.split_once('?').and_then(|(_, q)| {
                form_urlencoded::parse(q.as_bytes())
                    .find(|(k, _)| k == "next")
                    .map(|(_, v)| v.into_owned())
            });
            maybe_next.unwrap_or_else(|| incoming_url.to_string())
        }
    };

    match Url::parse(auth_service_url) {
        Ok(mut url) => {
            if !url.query_pairs().any(|(k, _)| k == "next") {
                url.query_pairs_mut().append_pair("next", &next_value);
            }
            Ok(url.to_string())
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let encoded_next = form_urlencoded::Serializer::new(String::new())
                .append_pair("next", &next_value)
                .finish();

            let (base, fragment) = auth_service_url
                .split_once('#')
                .map(|(b, f)| (b, Some(f)))
                .unwrap_or_else(|| (auth_service_url, None));

            let (path, query) = base
                .split_once('?')
                .map(|(p, q)| (p, Some(q)))
                .unwrap_or_else(|| (base, None));

            let mut redirect = String::from(path);

            match query {
                Some(q) => {
                    let has_next = form_urlencoded::parse(q.as_bytes()).any(|(k, _)| k == "next");
                    redirect.push('?');
                    redirect.push_str(q);
                    if !has_next {
                        if !q.is_empty() {
                            redirect.push('&');
                        }
                        redirect.push_str(&encoded_next);
                    }
                }
                None => {
                    redirect.push('?');
                    redirect.push_str(&encoded_next);
                }
            }

            if let Some(fragment) = fragment {
                redirect.push('#');
                redirect.push_str(fragment);
            }

            Ok(redirect)
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError(
            "Invalid auth service URL",
        )),
    }
}

/// Calls the wrapped service and redirects to the authentication service if
/// a `401 Unauthorized` response is encountered.
impl<S, B> Service<ServiceRequest> for RedirectUnauthorizedMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let server_config = req.app_data::<web::Data<CommonServerConfig>>();

        let auth_service_url = match server_config {
            Some(config) => config.auth_service_url.clone(),
            None => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorInternalServerError(
                        "Server config not found",
                    ))
                });
            }
        };

        // Record the full incoming URL before moving the request
        let incoming_url = format!(
            "{}://{}{}",
            req.connection_info().scheme(),
            req.connection_info().host(),
            req.uri()
        );

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            if res.status() == StatusCode::UNAUTHORIZED {
                let (req_parts, _) = res.into_parts();

                let redirect_url = build_redirect_url(&auth_service_url, &incoming_url)?;

                let redirect_response = HttpResponse::SeeOther()
                    .insert_header((actix_web::http::header::LOCATION, redirect_url))
                    .finish()
                    .map_into_right_body();

                return Ok(ServiceResponse::new(req_parts, redirect_response));
            }

            Ok(res.map_into_left_body())
        })
    }
}
