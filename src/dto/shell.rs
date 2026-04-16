use serde::{Deserialize, Serialize};

use crate::domain::auth::AuthenticatedUser;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CurrentUserDto {
    pub email: String,
    pub name: String,
    pub hub_id: i32,
    pub roles: Vec<String>,
}

impl From<AuthenticatedUser> for CurrentUserDto {
    fn from(user: AuthenticatedUser) -> Self {
        Self {
            email: user.email,
            name: user.name,
            hub_id: user.hub_id,
            roles: user.roles,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NavigationItemDto {
    pub name: String,
    pub url: String,
}

/// Shared shell payload for React-owned auth pages.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct IamDto {
    pub current_user: CurrentUserDto,
    pub home_url: String,
    pub navigation: Vec<NavigationItemDto>,
    pub local_menu_items: Vec<NavigationItemDto>,
    pub hub_name: String,
}

/// Minimal page-data payload for the CRM no-access page.
#[derive(Debug, Serialize)]
pub struct NoAccessPageDto {
    pub current_user: CurrentUserDto,
    pub home_url: String,
    pub required_role: Option<String>,
}

