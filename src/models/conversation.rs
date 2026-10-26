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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_conversation_should_have_no_messages() {
        let conversation = Conversation::new(
            1, 
            String::from("+50683032235"), 
            String::from("Juan")
        );
        assert_eq!(conversation.message_count(), 0);
    }

    #[test]
    fn add_message_should_increase_message_count() {
        let mut conversation = Conversation::new(
            1, 
            String::from("+50683032235"), 
            String::from("Juan")
        );
        let message = Message {
            phone_number: String::from("+50683032235"),
            sender: String::from("Juan"),
            content: String::from("Hola, ¿cómo estás?"),
            status: MessageStatus::Sending,
            attachment: None,
        };
        conversation.add_message(message);
        assert_eq!(conversation.message_count(), 1);
    }

    #[test]
    fn unread_count_should_return_correct_number_of_unread_messages() {
        let mut conversation = Conversation::new(
            1, 
            String::from("+50683032235"), 
            String::from("Juan")
        );
        let message1 = Message {
            phone_number: String::from("+50683032235"),
            sender: String::from("Juan"),
            content: String::from("Hola, ¿cómo estás?"),
            status: MessageStatus::Sending,
            attachment: None,
        };
        let message2 = Message {
            phone_number: String::from("+50683032235"),
            sender: String::from("Juan"),
            content: String::from("¿Qué tal tu día?"),
            status: MessageStatus::Read,
            attachment: None,
        };
        conversation.add_message(message1);
        conversation.add_message(message2);
        assert_eq!(conversation.unread_count(), 1);
    }
}