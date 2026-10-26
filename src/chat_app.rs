use crate::models::conversation::Conversation;
use crate::models::message::{Message, MessageStatus};

pub struct ChatApp {
    pub conversations: Vec<Conversation>,
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }

    pub fn add_conversation(&mut self, phone_number: String, name: String) {
        let id = self
            .conversations
            .iter()
            .map(|conversation| conversation.id)
            .max()
            .map_or(0, |last_id| last_id + 1);

        self.conversations
            .push(Conversation::new(id, phone_number, name));
    }

    pub fn add_message(&mut self, message: Message) {
        let conversation_index = self
            .conversations
            .iter()
            .position(|conversation| conversation.phone_number == message.phone_number);

        let conversation_index = match conversation_index {
            Some(index) => index,
            None => {
                self.add_conversation(message.phone_number.clone(), message.sender.clone());
                self.conversations.len() - 1
            }
        };

        self.conversations[conversation_index].add_message(message);
    }

    pub fn total_messages(&self) -> usize {
        self.conversations.iter().map(|c| c.message_count()).sum()
    }

    pub fn total_unread_messages(&self) -> usize {
        self.conversations.iter().map(|c| c.unread_count()).sum()
    }

    pub fn find_conversation(&self, phone_number: &str) -> Option<&Conversation> {
        self.conversations
            .iter()
            .find(|c| c.phone_number == phone_number)
    }

    pub fn find_conversation_mut(&mut self, phone_number: &str) -> Option<&mut Conversation> {
        self.conversations
            .iter_mut()
            .find(|c| c.phone_number == phone_number)
    }

    pub fn send_message(
        &mut self,
        phone_number: &str,
        content: &str,
        attachment: Option<&str>,
    ) -> Result<(), String> {
        if content.trim().is_empty() {
            return Err(String::from("El mensaje no puede estar vacío."));
        }

        let conversation = self
            .find_conversation_mut(phone_number)
            .ok_or(String::from("Conversacion no encontrada"))?;

        conversation.add_message(Message {
            phone_number: String::from(phone_number),
            sender: String::from("Yo"),
            content: String::from(content),
            status: MessageStatus::Sending,
            attachment: attachment.map(String::from),
        });

        Ok(())
    }
}
