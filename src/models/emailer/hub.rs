use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::emailer::hub::{
    Hub as DomainHub, NewHub as DomainNewHub, UpdateHub as DomainUpdateHub,
};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::emailer::hubs)]
pub struct Hub {
    pub id: i32,
    pub login: Option<String>,
    pub password: Option<String>,
    pub sender: Option<String>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub imap_server: Option<String>,
    pub imap_port: Option<i32>,
    pub email_template: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::emailer::hubs)]
pub struct NewHub<'a> {
    pub id: i32,
    pub login: Option<&'a str>,
    pub password: Option<&'a str>,
    pub sender: Option<&'a str>,
    pub smtp_server: Option<&'a str>,
    pub smtp_port: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub imap_server: Option<&'a str>,
    pub imap_port: Option<i32>,
    pub email_template: Option<&'a str>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::emailer::hubs)]
pub struct UpdateHub<'a> {
    pub login: Option<&'a str>,
    pub password: Option<&'a str>,
    pub sender: Option<&'a str>,
    pub smtp_server: Option<&'a str>,
    pub smtp_port: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub imap_server: Option<&'a str>,
    pub imap_port: Option<i32>,
    pub email_template: Option<&'a str>,
}

impl From<Hub> for DomainHub {
    fn from(value: Hub) -> Self {
        Self {
            id: value.id,
            login: value.login,
            password: value.password,
            sender: value.sender,
            smtp_server: value.smtp_server,
            smtp_port: value.smtp_port,
            created_at: value.created_at,
            updated_at: value.updated_at,
            imap_server: value.imap_server,
            imap_port: value.imap_port,
            email_template: value.email_template,
        }
    }
}

impl<'a> From<&'a DomainNewHub> for NewHub<'a> {
    fn from(value: &'a DomainNewHub) -> Self {
        Self {
            id: value.id,
            login: value.login.as_deref(),
            password: value.password.as_deref(),
            sender: value.sender.as_deref(),
            smtp_server: value.smtp_server.as_deref(),
            smtp_port: value.smtp_port,
            created_at: value.created_at,
            updated_at: value.updated_at,
            imap_server: value.imap_server.as_deref(),
            imap_port: value.imap_port,
            email_template: value.email_template.as_deref(),
        }
    }
}

impl<'a> From<&'a DomainUpdateHub> for UpdateHub<'a> {
    fn from(value: &'a DomainUpdateHub) -> Self {
        Self {
            login: value.login.as_deref(),
            password: value.password.as_deref(),
            sender: value.sender.as_deref(),
            smtp_server: value.smtp_server.as_deref(),
            smtp_port: value.smtp_port,
            created_at: value.created_at,
            updated_at: value.updated_at,
            imap_server: value.imap_server.as_deref(),
            imap_port: value.imap_port,
            email_template: value.email_template.as_deref(),
        }
    }
}
