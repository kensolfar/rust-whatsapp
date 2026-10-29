use crate::models::conversation::Conversation;
use crate::models::message::{Message, MessageDirection, MessageStatus};

const MY_PHONE_NUMBER: &str = "+50683032235";

fn is_empty_message(content: &str, attachment: Option<&str>) -> bool {
    content.trim().is_empty() && attachment.is_none_or(|a| a.trim().is_empty())
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChatError {
    EmptyMessage,
    ConversationNotFound,
    ConversationAlreadyExists,
}

impl std::fmt::Display for ChatError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::EmptyMessage => "El mensaje no puede estar vacío.",
            Self::ConversationNotFound => "La conversación no existe.",
            Self::ConversationAlreadyExists => "La conversación ya existe.",
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

    fn create_conversation(&mut self, phone_number: String, name: String) {
        let id = self
            .conversations
            .iter()
            .map(|conversation| conversation.id())
            .max()
            .map_or(0, |last_id| last_id + 1);

        self.conversations
            .push(Conversation::new(id, phone_number, name));
    }

    pub fn add_conversation(
        &mut self,
        phone_number: String,
        name: String,
    ) -> Result<(), ChatError> {
        let conversation_exists = self
            .conversations
            .iter()
            .any(|conversation| conversation.phone_number() == phone_number);
        if conversation_exists {
            return Err(ChatError::ConversationAlreadyExists);
        }
        self.create_conversation(phone_number, name);
        Ok(())
    }

    pub fn add_incoming_message(
        &mut self,
        phone_number: &str,
        sender: &str,
        content: &str,
        attachment: Option<&str>,
    ) -> Result<(), ChatError> {

        if is_empty_message(content, attachment) {
            return Err(ChatError::EmptyMessage);
        }

        let conversation_index = self
            .conversations
            .iter()
            .position(|conversation| conversation.phone_number() == phone_number);

        let conversation_index = match conversation_index {
            Some(index) => index,
            None => {
                let _ =
                    self.add_conversation(String::from(phone_number).to_owned(), String::from(sender));
                self.conversations.len() - 1
            }
        };

        self.conversations[conversation_index].add_message(Message::new(
            String::from(phone_number),
            String::from(sender),
            String::from(content),
            MessageStatus::Sending,
            MessageDirection::Incoming,
            attachment.map(String::from),
            false,
        ));

        Ok(())
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
        if is_empty_message(content, attachment) {
            return Err(ChatError::EmptyMessage);
        }

        let conversation = self
            .find_conversation_mut(phone_number)
            .ok_or(ChatError::ConversationNotFound)?;

        conversation.add_message(Message::new(
            String::from(MY_PHONE_NUMBER),
            String::from("Yo"),
            String::from(content),
            MessageStatus::Sending,
            MessageDirection::Outgoing,
            attachment.map(String::from),
            false,
        ));

        Ok(())
    }

    pub fn mark_conversation_as_read(&mut self, phone_number: &str) -> Result<(), ChatError> {
        let conversation = self
            .find_conversation_mut(phone_number)
            .ok_or(ChatError::ConversationNotFound)?;

        conversation.mark_as_read();

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
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();
        assert_eq!(chat_app.conversations.len(), 1);
    }

    #[test]
    fn add_conversation_should_fail_for_an_existing_phone_number() {
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();

        let result = chat_app.add_conversation(String::from("+50683032235"), String::from("Pedro"));

        assert_eq!(result, Err(ChatError::ConversationAlreadyExists));
        assert_eq!(chat_app.conversation_count(), 1);
    }

    #[test]
    fn find_conversation_should_return_correct_conversation() {
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();
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
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();
        let conversation = chat_app.find_conversation_mut("+50683032235");
        assert!(conversation.is_some());
        assert_eq!(conversation.unwrap().name(), "Juan");
    }

    #[test]
    fn find_conversation_mut_should_allow_modification() {
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();
        let conversation = chat_app.find_conversation_mut("+50683032235");
        assert!(conversation.is_some());
        conversation.unwrap().set_name(String::from("Pedro"));
        let updated_conversation = chat_app.find_conversation("+50683032235");
        assert_eq!(updated_conversation.unwrap().name(), "Pedro");
    }
    
    #[test]
    fn send_message_should_add_outgoing_message_to_conversation() {
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();
        chat_app
            .send_message("+50683032235", "Hola", None)
            .unwrap();
        let conversation = chat_app.find_conversation("+50683032235").unwrap();
        assert_eq!(conversation.message_count(), 1);
        assert_eq!(conversation.messages()[0].content(), "Hola");
        assert_eq!(conversation.messages()[0].direction(), &MessageDirection::Outgoing);
    }

    #[test]
    fn send_message_should_not_increase_unread_count() {
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from("+50683032235"), String::from("Juan"))
            .unwrap();

        chat_app
            .send_message("+50683032235", "Hola", None)
            .unwrap();

        assert_eq!(chat_app.total_unread_messages(), 0);
    }

    #[test]
    fn add_incoming_message_should_increase_unread_count() {
        let mut chat_app = ChatApp::new();
        assert_eq!(chat_app.total_unread_messages(), 0);

        chat_app
            .add_incoming_message("+50683032235", "Juan", "Hola", None)
            .unwrap();

        assert_eq!(chat_app.total_unread_messages(), 1);
    }

    #[test]
    fn mark_conversation_as_read_should_mark_all_incoming_messages_as_read() {
        const PHONE_NUMBER: &str = "+50688888888";
        let mut chat_app = ChatApp::new();
        chat_app
            .add_conversation(String::from(PHONE_NUMBER), String::from("Juan"))
            .unwrap();
        chat_app.add_incoming_message(
            PHONE_NUMBER,
            "Juan",
            "Hola",
            None,
        ).unwrap();
        chat_app.send_message(PHONE_NUMBER, "Hola", None).unwrap();
        chat_app.add_incoming_message(
            PHONE_NUMBER,
            "Juan",
            "¿Cómo estás?",
            None,
        ).unwrap();
        chat_app.send_message(PHONE_NUMBER, "Muy bien, gracias.", None).unwrap();
        chat_app
            .mark_conversation_as_read(PHONE_NUMBER)
            .unwrap();
        let conversation = chat_app.find_conversation(PHONE_NUMBER).unwrap();
        assert_eq!(conversation.unread_count(), 0);
    }

    #[test]
    fn mark_nonexistent_conversation_as_read_should_return_not_found_error() {
        let mut chat_app = ChatApp::new();

        let result = chat_app.mark_conversation_as_read("+50688888888");

        assert_eq!(result, Err(ChatError::ConversationNotFound));
    }
}
