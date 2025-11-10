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

impl ZMQSendSmsMessage {
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.sender_id.is_empty() {
            return Err(ServiceError::InvalidMessage("sender_id is empty".into()));
        }
        let phone = parse(None, &self.phone_number)?;
        if !phone.is_valid() {
            return Err(ServiceError::InvalidPhoneNumber(
                phonenumber::ParseError::NoNumber,
            ));
        }
        if self.message.is_empty() {
            return Err(ServiceError::InvalidMessage("message is empty".into()));
        }
        Ok(())
    }

    pub fn mask_phone(&self) -> String {
        let mut s = self.phone_number.chars();

        let keep_start = 4;

        let mut out = String::new();
        out.extend(s.by_ref().take(keep_start));
        out.extend(std::iter::repeat_n('*', 6));
        out.extend(s.skip(6));
        out
    }
}
