use crate::models::conversation::Conversation;
use crate::models::message::{Message, MessageStatus};

#[derive(Debug, PartialEq, Eq)]
pub enum ChatError {
    EmptyMessage,
    ConversationNotFound,
}

impl std::fmt::Display for ChatError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyMessage => "El mensaje no puede estar vacío.",
            Self::ConversationNotFound => "La conversación no existe.",
        };

        formatter.write_str(message)
    }
}

impl std::error::Error for ChatError {}

pub struct ChatApp {
    conversations: Vec<Conversation>,
}

impl ChatApp {
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }

    pub fn conversations(&self) -> &[Conversation] {
        &self.conversations
    }

    pub fn conversation_count(&self) -> usize {
        self.conversations.len()
    }

    pub fn add_conversation(&mut self, phone_number: String, name: String) {
        let id = self
            .conversations
            .iter()
            .map(|conversation| conversation.id())
            .max()
            .map_or(0, |last_id| last_id + 1);

        self.conversations
            .push(Conversation::new(id, phone_number, name));
    }

    pub fn add_message(&mut self, message: Message) {
        let conversation_index = self
            .conversations
            .iter()
            .position(|conversation| conversation.phone_number() == message.phone_number());

        let conversation_index = match conversation_index {
            Some(index) => index,
            None => {
                self.add_conversation(message.phone_number().into(), message.sender().into());
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
            .find(|c| c.phone_number() == phone_number)
    }

    pub fn find_conversation_mut(&mut self, phone_number: &str) -> Option<&mut Conversation> {
        self.conversations
            .iter_mut()
            .find(|c| c.phone_number() == phone_number)
    }

    pub fn send_message(
        &mut self,
        phone_number: &str,
        content: &str,
        attachment: Option<&str>,
    ) -> Result<(), ChatError> {
        if content.trim().is_empty() && attachment.is_none_or(|a| a.trim().is_empty()) {
            return Err(ChatError::EmptyMessage);
        }

        let conversation = self
            .find_conversation_mut(phone_number)
            .ok_or(ChatError::ConversationNotFound)?;

        conversation.add_message(Message::new(
            String::from(phone_number),
            String::from("Yo"),
            String::from(content),
            MessageStatus::Sending,
            attachment.map(String::from),
        ));

        Ok(())
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_chat_app_should_have_no_conversations() {
        let chat_app = ChatApp::new();
        assert_eq!(chat_app.conversations.len(), 0);
    }

    #[test]
    fn add_conversation_should_increase_conversation_count() {
        let mut chat_app = ChatApp::new();
        chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));
        assert_eq!(chat_app.conversations.len(), 1);
    }

    #[test]
    fn find_conversation_should_return_correct_conversation() {
        let mut chat_app = ChatApp::new();
        chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));
        let conversation = chat_app.find_conversation("+50683032235");
        assert!(conversation.is_some());
        assert_eq!(conversation.unwrap().name(), "Juan");
    }

    #[test]
    fn find_conversation_should_return_null_if_not_found() {
        let chat_app = ChatApp::new();
        let conversation = chat_app.find_conversation("+50683032235");
        assert!(conversation.is_none());
    }

    #[test]
    fn find_conversation_mut_should_return_correct_conversation() {
        let mut chat_app = ChatApp::new();
        chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));
        let conversation = chat_app.find_conversation_mut("+50683032235");
        assert!(conversation.is_some());
        assert_eq!(conversation.unwrap().name(), "Juan");
    }

    #[test]
    fn find_conversation_mut_should_allow_modification() {
        let mut chat_app = ChatApp::new();
        chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));
        let conversation = chat_app.find_conversation_mut("+50683032235");
        assert!(conversation.is_some());
        conversation.unwrap().set_name(String::from("Pedro"));
        let updated_conversation = chat_app.find_conversation("+50683032235");
        assert_eq!(updated_conversation.unwrap().name(), "Pedro");
    }
}
