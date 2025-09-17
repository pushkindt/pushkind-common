use serde::{Deserialize, Serialize};

use crate::domain::auth::AuthenticatedUser;
use crate::domain::emailer::email::NewEmail;

#[derive(Serialize, Deserialize)]
pub enum ZMQSendEmailMessage {
    NewEmail(Box<(AuthenticatedUser, NewEmail)>),
    RetryEmail((i32, i32)), // (id, hub_id)
}

#[derive(Serialize, Deserialize)]
pub struct ZMQReplyMessage {
    pub hub_id: i32,
    pub email: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZMQUnsubscribeMessage {
    pub hub_id: i32,
    pub email: String,
    pub reason: Option<String>,
}
