use crate::chat_app::ChatApp;
use crate::models::conversation::Conversation;
use crate::models::message::{Message, MessageStatus};

pub fn print_chats(chat: &ChatApp) {
    print!("{}", render_chats(chat));
}

fn render_status(status: Option<&MessageStatus>) -> &'static str {
    match status {
        Some(MessageStatus::Sending) => "Estado: Enviando",
        Some(MessageStatus::Sent) => "Estado: Enviado",
        Some(MessageStatus::Delivered) => "Estado: Entregado",
        Some(MessageStatus::Read) => "Estado: Leído",
        Some(MessageStatus::Failed) => "Estado: Fallido",
        None => "Mensaje Entrante"
    }
}

fn render_attachment(attachment: Option<&str>) -> String {
    match attachment {
        Some(file) => format!("Archivo adjunto: {file}"),
        None => String::from("No hay archivo adjunto"),
    }
}

pub fn render_message(message: &Message) -> String {
    format!(
        "De: {}\nMensaje: {}\n{}\n{}\n-------------------------\n",
        message.sender(),
        message.content(),
        render_status(message.status()),
        render_attachment(message.attachment()),
    )
}

pub fn render_conversation_stats(conversation: &Conversation) -> String {
    format!(
        "{}\nTeléfono: {}\n-------------------------\nTotal de mensajes: {}\nMensajes no leidos: {}\n\n",
        conversation.name(),
        conversation.phone_number(),
        conversation.message_count(),
        conversation.unread_count(),
    )
}

pub fn render_chats(chat: &ChatApp) -> String {
    let mut output = format!(
        "\n=== {} Conversaciones ===\n\nNo leidos: {}\n\n",
        chat.conversation_count(),
        chat.total_unread_messages(),
    );

    for conversation in chat.conversations() {
        output.push_str(&render_conversation_stats(conversation));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::message::MessageDirection;

    fn message(status: Option<MessageStatus>, attachment: Option<&str>) -> Message {
        Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola"),
            status,
            MessageDirection::Outgoing,
            attachment.map(String::from),
            false,
        )
    }

    #[test]
    fn render_status_should_return_the_expected_label() {
        assert_eq!(render_status(Some(&MessageStatus::Sending)), "Estado: Enviando");
        assert_eq!(render_status(Some(&MessageStatus::Sent)), "Estado: Enviado");
        assert_eq!(
            render_status(Some(&MessageStatus::Delivered)),
            "Estado: Entregado"
        );
        assert_eq!(render_status(Some(&MessageStatus::Read)), "Estado: Leído");
        assert_eq!(render_status(Some(&MessageStatus::Failed)), "Estado: Fallido");
    }

    #[test]
    fn render_status_should_label_messages_without_status_as_incoming() {
        assert_eq!(render_status(None), "Mensaje Entrante");
    }

    #[test]
    fn render_attachment_should_handle_an_attachment() {
        assert_eq!(
            render_attachment(Some("imagen.jpg")),
            "Archivo adjunto: imagen.jpg"
        );
    }

    #[test]
    fn render_attachment_should_handle_missing_attachment() {
        assert_eq!(render_attachment(None), "No hay archivo adjunto");
    }

    #[test]
    fn render_message_should_include_message_details() {
        let rendered = render_message(&message(Some(MessageStatus::Sent), Some("imagen.jpg")));

        assert!(rendered.contains("De: Juan"));
        assert!(rendered.contains("Mensaje: Hola"));
        assert!(rendered.contains("Estado: Enviado"));
        assert!(rendered.contains("Archivo adjunto: imagen.jpg"));
    }

    #[test]
    fn render_chats_should_include_conversation_statistics() {
        let mut chat = ChatApp::new();
        chat.add_incoming_message(
            "PHONE_NUMBER",
            "Juan",
            "¿Cómo estás?",
            None,).unwrap();

        let rendered = render_chats(&chat);

        assert!(rendered.contains("=== 1 Conversaciones ==="));
        assert!(rendered.contains("No leidos: 1"));
        assert!(rendered.contains("Total de mensajes: 1"));
        assert!(rendered.contains("Mensajes no leidos: 1"));
    }
}
