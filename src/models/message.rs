#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[derive(PartialEq)]
pub struct Message {
    phone_number: String,
    sender: String,
    content: String,
    status: MessageStatus,
    direction: MessageDirection,
    attachment: Option<String>,
    is_read: bool,
}

impl Message {
    pub fn new(
        phone_number: String,
        sender: String,
        content: String,
        status: MessageStatus,
        direction: MessageDirection,
        attachment: Option<String>,
        is_read: bool,
    ) -> Self {
        Self {
            phone_number,
            sender,
            content,
            status,
            direction,
            attachment,
            is_read,
        }
    }
    pub fn sender(&self) -> &str {
        &self.sender
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn status(&self) -> &MessageStatus {
        &self.status
    }

    pub fn direction(&self) -> &MessageDirection {
        &self.direction
    }

    pub fn attachment(&self) -> Option<&str> {
        self.attachment.as_deref()
    }

    pub fn update_status(&mut self, new_status: MessageStatus) {
        if self.direction != MessageDirection::Incoming {
            self.status = new_status;
        }
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    pub fn is_read(&self) -> bool {
        self.is_read
    }

    pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(attachment: Option<&str>) -> Message {
        Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            MessageStatus::Sending,
            MessageDirection::Incoming,
            attachment.map(String::from),
            false,
        )
    }

    #[test]
    fn new_message_no_attachment_should_not_have_attachment() {
        let message = message(None);

        assert!(!message.has_attachment());
    }

    #[test]
    fn new_message_with_attachment_should_have_attachment() {
        let message = message(Some("image.png"));
        assert!(message.has_attachment());
    }

    #[test]
    fn new_message_should_have_correct_status() {
        let message = message(None);

        assert_eq!(message.status, MessageStatus::Sending);
    }

    #[test]
    fn update_status_should_change_message_status() {
        let mut message = Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            MessageStatus::Sending,
            MessageDirection::Outgoing,
            None,
            false,
        );
        message.update_status(MessageStatus::Sent);
        assert_eq!(message.status, MessageStatus::Sent);
    }
}
