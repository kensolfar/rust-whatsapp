#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}
#[derive(PartialEq)]
pub struct Message {
    pub phone_number: String,
    pub sender: String,
    pub content: String,
    pub status: MessageStatus,
    pub attachment: Option<String>,
}

impl Message {
    pub fn update_status(&mut self, new_status: MessageStatus) {
        self.status = new_status;
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(attachment: Option<&str>) -> Message {
        Message {
            phone_number: String::from("+50683032235"),
            sender: String::from("Juan"),
            content: String::from("Hola, ¿cómo estás?"),
            status: MessageStatus::Sending,
            attachment: attachment.map(String::from),
        }
    }

    #[test]
    fn new_message_no_attachment_should_not_have_attachment() {
        let message = message(None);

        assert_eq!(message.has_attachment(), false);
    }

    #[test]
    fn new_message_with_attachment_should_have_attachment() {
        let message = message(Some("image.png"));
        assert_eq!(message.has_attachment(), true);
    }

    #[test]
    fn new_message_should_have_correct_status() {
        let message = message(None);

        assert_eq!(message.status, MessageStatus::Sending);
    }

    #[test]
    fn update_status_should_change_message_status() {
        let mut message = message(None);
        message.update_status(MessageStatus::Sent);
        assert_eq!(message.status, MessageStatus::Sent);
    }
}
