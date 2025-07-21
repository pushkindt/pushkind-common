#[derive(Clone)]
/// Configuration shared across different services.
///
/// - `secret` is used to sign and verify JWT tokens.
/// - `auth_service_url` is where unauthorized users are redirected for
///   authentication.
pub struct CommonServerConfig {
    pub secret: String,
    pub auth_service_url: String,
}
