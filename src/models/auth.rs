use std::future::{Ready, ready};

use actix_identity::Identity;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, web::Data};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, encode};

use crate::domain::auth::AuthenticatedUser;
use crate::models::config::CommonServerConfig;

impl AuthenticatedUser {
    /// Set the `exp` claim to the current time plus the provided number of days.
    pub fn set_expiration(&mut self, days: i64) {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(days))
            .expect("valid timestamp")
            .timestamp() as usize;
        self.exp = expiration;
    }

    /// Encode this user into a JWT using the given secret key.
    ///
    /// The expiration is automatically set to 7 days from now.
    pub fn to_jwt(&mut self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        self.set_expiration(7);
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
    }
    /// Decode a JWT and return the contained claims.
    pub fn from_jwt(token: &str, secret: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let validation = jsonwebtoken::Validation::default();
        let token_data = jsonwebtoken::decode::<Self>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = Identity::from_request(req, &mut Payload::None)
            .into_inner()
            .map(|i| i.id().ok());

        let server_config = req.app_data::<Data<CommonServerConfig>>();

        let server_config = match server_config {
            Some(config) => config,
            None => return ready(Err(ErrorInternalServerError("Server config not found"))),
        };

        if let Ok(Some(uid)) = identity {
            let claims = AuthenticatedUser::from_jwt(&uid, &server_config.secret);

            match claims {
                Ok(claims) => return ready(Ok(claims)),
                Err(_) => return ready(Err(ErrorUnauthorized("Invalid user"))),
            };
        }
        ready(Err(ErrorUnauthorized("Unauthorized")))
    }
}

// NOTE: Implementing `FromRequest` allows `AuthenticatedUser` to be extracted
// directly from an incoming `HttpRequest` in Actix handlers.

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation, decode};

    fn sample_user() -> AuthenticatedUser {
        AuthenticatedUser {
            sub: "1".to_string(),
            email: "test@example.com".to_string(),
            hub_id: 1,
            name: "Test".to_string(),
            roles: vec!["crm".to_string()],
            exp: 0,
        }
    }

    #[test]
    fn jwt_round_trip() {
        let secret = "secret";
        let mut user = sample_user();
        let token = user.to_jwt(secret).unwrap();
        let decoded = decode::<AuthenticatedUser>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap()
        .claims;

        assert_eq!(decoded.sub, user.sub);
        assert_eq!(decoded.email, user.email);
        assert_eq!(decoded.hub_id, user.hub_id);
        assert_eq!(decoded.name, user.name);
        assert_eq!(decoded.roles, user.roles);
        assert_eq!(decoded.exp, user.exp);
    }
}
