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
use url::Url;

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

                let redirect_url = match Url::parse(&auth_service_url) {
                    Ok(mut url) => {
                        url.query_pairs_mut().append_pair("next", &incoming_url);
                        url.to_string()
                    }
                    Err(_) => {
                        return Err(actix_web::error::ErrorInternalServerError(
                            "Invalid auth service URL",
                        ));
                    }
                };

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
