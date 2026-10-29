use crate::models::conversation::Conversation;
use crate::models::message::{Message, MessageDirection, MessageStatus};

const MY_PHONE_NUMBER: &str = "+50683032235";

/// Indica si el contenido y el archivo adjunto están vacíos.
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
    /// Escribe el mensaje legible asociado al error.
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
    /// Crea una aplicación de chat sin conversaciones.
    pub fn new() -> Self {
        Self {
            conversations: Vec::new(),
        }
    }

    /// Devuelve todas las conversaciones almacenadas en el orden de creación.
    pub fn conversations(&self) -> &[Conversation] {
        &self.conversations
    }

    /// Devuelve la cantidad de conversaciones almacenadas.
    pub fn conversation_count(&self) -> usize {
        self.conversations.len()
    }

    /// Crea una conversación y le asigna el siguiente identificador disponible.
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

    /// Agrega una conversación nueva.
    ///
    /// Devuelve [`ChatError::ConversationAlreadyExists`] si ya existe una
    /// conversación con el mismo número telefónico.
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

    /// Agrega un mensaje entrante a la conversación indicada.
    ///
    /// Si la conversación no existe, la crea usando `sender` como nombre.
    /// Devuelve [`ChatError::EmptyMessage`] cuando no hay contenido ni archivo
    /// adjunto.
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
                    self.add_conversation(String::from(phone_number), String::from(sender));
                self.conversations.len() - 1
            }
        };

        self.conversations[conversation_index].add_message(Message::new(
            String::from(phone_number),
            String::from(sender),
            String::from(content),
            None,
            MessageDirection::Incoming,
            attachment.map(String::from),
            false,
        ));

        Ok(())
    }

    /// Devuelve la cantidad total de mensajes de todas las conversaciones.
    pub fn total_messages(&self) -> usize {
        self.conversations.iter().map(|c| c.message_count()).sum()
    }

    /// Devuelve la cantidad total de mensajes entrantes no leídos.
    pub fn total_unread_messages(&self) -> usize {
        self.conversations.iter().map(|c| c.unread_count()).sum()
    }

    /// Busca una conversación por número telefónico.
    pub fn find_conversation(&self, phone_number: &str) -> Option<&Conversation> {
        self.conversations
            .iter()
            .find(|c| c.phone_number() == phone_number)
    }

    /// Busca una conversación mutable por número telefónico.
    pub fn find_conversation_mut(&mut self, phone_number: &str) -> Option<&mut Conversation> {
        self.conversations
            .iter_mut()
            .find(|c| c.phone_number() == phone_number)
    }

    /// Agrega un mensaje saliente a una conversación existente.
    ///
    /// El mensaje se crea con estado [`MessageStatus::Sending`]. Devuelve
    /// [`ChatError::EmptyMessage`] si no hay contenido ni archivo adjunto, o
    /// [`ChatError::ConversationNotFound`] si no existe la conversación.
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
            Some(MessageStatus::Sending),
            MessageDirection::Outgoing,
            attachment.map(String::from),
            true,
        ));

        Ok(())
    }

    /// Marca como leídos todos los mensajes entrantes de una conversación.
    pub fn mark_conversation_as_read(&mut self, phone_number: &str) -> Result<(), ChatError> {
        let conversation = self
            .find_conversation_mut(phone_number)
            .ok_or(ChatError::ConversationNotFound)?;

        conversation.mark_as_read();

        Ok(())
    }
}

impl Default for ChatApp {
    /// Crea una aplicación de chat sin conversaciones.
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
        let conversation = chat_app.find_conversation("+50683032235").unwrap();
        assert_eq!(conversation.messages()[0].status(), None);
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
