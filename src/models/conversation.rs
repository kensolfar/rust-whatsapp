use super::message::{Message, MessageDirection};

pub struct Conversation {
    id: u64,
    phone_number: String,
    name: String,
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new(id: u64, phone_number: String, name: String) -> Self {
        Self {
            id,
            phone_number,
            name,
            messages: Vec::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn mark_as_read(&mut self) {
        for message in &mut self.messages {
            if message.direction() == &MessageDirection::Incoming {
                message.mark_as_read();
            }
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn unread_count(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| !m.is_read() && m.direction() == &MessageDirection::Incoming)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::message::MessageStatus;

    fn conversation() -> Conversation {
        Conversation::new(1, String::from("+50683032235"), String::from("Juan"))
    }

    fn message(direction: MessageDirection, is_read: bool, content: &str) -> Message {
        Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from(content),
            MessageStatus::Sending,
            direction,
            None,
            is_read,
        )
    }

    #[test]
    fn new_conversation_should_store_identity_and_have_no_messages() {
        let conversation = conversation();

        assert_eq!(conversation.id(), 1);
        assert_eq!(conversation.phone_number(), "+50683032235");
        assert_eq!(conversation.name(), "Juan");
        assert_eq!(conversation.message_count(), 0);
        assert!(conversation.messages().is_empty());
    }

    #[test]
    fn set_name_should_replace_existing_name() {
        let mut conversation = conversation();

        conversation.set_name(String::from("Pedro"));

        assert_eq!(conversation.name(), "Pedro");
    }

    #[test]
    fn add_message_should_preserve_order_and_increase_message_count() {
        let mut conversation = conversation();
        let first = message(MessageDirection::Outgoing, false, "First");
        let second = message(MessageDirection::Incoming, false, "Second");

        conversation.add_message(first);
        conversation.add_message(second);

        assert_eq!(conversation.message_count(), 2);
        assert_eq!(conversation.messages()[0].content(), "First");
        assert_eq!(conversation.messages()[1].content(), "Second");
    }

    #[test]
    fn mark_as_read_should_mark_incoming_messages_without_changing_outgoing_messages() {
        let mut conversation = conversation();

        conversation.add_message(message(MessageDirection::Incoming, false, "Incoming"));
        conversation.add_message(message(MessageDirection::Outgoing, false, "Outgoing"));

        conversation.mark_as_read();

        assert!(conversation.messages()[0].is_read());
        assert!(!conversation.messages()[1].is_read());
        assert_eq!(conversation.unread_count(), 0);
    }

    #[test]
    fn unread_count_should_only_include_unread_incoming_messages() {
        let mut conversation = conversation();

        conversation.add_message(message(MessageDirection::Incoming, false, "Unread incoming"));
        conversation.add_message(message(MessageDirection::Incoming, true, "Read incoming"));
        conversation.add_message(message(MessageDirection::Outgoing, false, "Unread outgoing"));
        conversation.add_message(message(MessageDirection::Outgoing, true, "Read outgoing"));

        assert_eq!(conversation.unread_count(), 1);
    }

    #[test]
    fn unread_count_should_be_zero_when_there_are_no_unread_incoming_messages() {
        let mut conversation = conversation();

        conversation.add_message(message(MessageDirection::Incoming, true, "Read incoming"));
        conversation.add_message(message(MessageDirection::Outgoing, false, "Unread outgoing"));

        assert_eq!(conversation.unread_count(), 0);
    }
}
