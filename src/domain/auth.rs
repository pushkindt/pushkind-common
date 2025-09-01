use serde::{Deserialize, Serialize};

/// Claims representing an authenticated user stored inside a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub sub: String, // subject (user ID or UUID)
    pub email: String,
    pub hub_id: i32,
    pub name: String,
    pub roles: Vec<String>,
    pub exp: usize, // expiration as timestamp
}
