use super::message::{Message, MessageStatus};
pub struct Conversation {
    pub id: u64,
    pub phone_number: String,
    pub name: String,
    pub messages: Vec<Message>,
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

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn unread_count(&self) -> usize {
        self.messages
            .iter()
            .filter(|m| m.status != MessageStatus::Read)
            .count()
    }
}
