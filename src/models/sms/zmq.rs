use phonenumber::parse;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// JSON payload representing a request to send a single SMS message.
///
/// The payload is provided by the ZeroMQ producer and must contain a
/// non-empty sender ID, a valid E.164-formatted phone number, and a message
/// body. Validation ensures these requirements before any publish occurs.
pub struct ZMQSendSmsMessage {
    pub sender_id: String,
    pub phone_number: String,
    pub message: String,
}

#[derive(Error, Debug)]
/// Errors that can occur while validating SMS jobs.
///
/// Each variant wraps the underlying library error when available so callers
/// receive structured context suitable for logging or retry decisions.
pub enum ZMQSendSmsValidationError {
    #[error("Invalid sender: {0}")]
    InvalidSender(String),
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    #[error("Invalid phone number")]
    InvalidPhoneNumber(#[from] phonenumber::ParseError),
}

impl ZMQSendSmsMessage {
    /// Validates the sender, recipient, and message payload before publishing.
    ///
    /// Returns a structured validation error so callers can decide on retry or
    /// logging strategies.
    pub fn validate(&self) -> Result<(), ZMQSendSmsValidationError> {
        if self.sender_id.is_empty() {
            return Err(ZMQSendSmsValidationError::InvalidSender(
                "sender_id is empty".into(),
            ));
        }
        let phone = parse(None, &self.phone_number)?;
        if !phone.is_valid() {
            return Err(ZMQSendSmsValidationError::InvalidPhoneNumber(
                phonenumber::ParseError::NoNumber,
            ));
        }
        if self.message.is_empty() {
            return Err(ZMQSendSmsValidationError::InvalidMessage(
                "message is empty".into(),
            ));
        }
        Ok(())
    }

    /// Returns a masked version of `phone_number` for logging, hiding the middle digits.
    pub fn mask_phone(&self) -> String {
        let mut s = self.phone_number.chars();

        let keep_start = 4;
        let asterisk_count = 6;

        let mut out = String::new();
        out.extend(s.by_ref().take(keep_start));
        out.extend(std::iter::repeat_n('*', asterisk_count));
        out.extend(s.skip(asterisk_count));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let valid = ZMQSendSmsMessage {
            sender_id: "test".into(),
            phone_number: "+14185438090".into(),
            message: "test".into(),
        };
        assert!(valid.validate().is_ok());

        let invalid_sender = ZMQSendSmsMessage {
            sender_id: "".into(),
            phone_number: "+1234567890".into(),
            message: "test".into(),
        };
        assert!(matches!(
            invalid_sender.validate(),
            Err(ZMQSendSmsValidationError::InvalidSender(_))
        ));

        let invalid_phone = ZMQSendSmsMessage {
            sender_id: "test".into(),
            phone_number: "1234567890".into(),
            message: "test".into(),
        };
        assert!(matches!(
            invalid_phone.validate(),
            Err(ZMQSendSmsValidationError::InvalidPhoneNumber(_))
        ));

        let invalid_message = ZMQSendSmsMessage {
            sender_id: "test".into(),
            phone_number: "+14185438090".into(),
            message: "".into(),
        };
        assert!(matches!(
            invalid_message.validate(),
            Err(ZMQSendSmsValidationError::InvalidMessage(_))
        ));
    }

    #[test]
    fn test_mask_phone() {
        let valid = ZMQSendSmsMessage {
            sender_id: "test".into(),
            phone_number: "+1234567890".into(),
            message: "test".into(),
        };
        assert_eq!(valid.mask_phone(), "+123******0");
    }
}
