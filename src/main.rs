mod chat_app;
mod models;

use chat_app::ChatApp;
use models::conversation::Conversation;
use models::message::{Message, MessageStatus};

fn print_status(status: &MessageStatus) {
    match status {
        MessageStatus::Sending => println!("Estado: Enviando"),
        MessageStatus::Sent => println!("Estado: Enviado"),
        MessageStatus::Delivered => println!("Estado: Entregado"),
        MessageStatus::Read => println!("Estado: Leído"),
        MessageStatus::Failed => println!("Estado: Fallido"),
    }
}

fn print_attachment(attachment: &Option<String>) {
    match attachment {
        Some(file) => println!("Archivo adjunto: {}", file),
        None => println!("No hay archivo adjunto"),
    }
}

fn print_message(message: &Message) {
    println!("De: {}", message.sender);
    println!("Mensaje: {}", message.content);
    print_status(&message.status);
    print_attachment(&message.attachment);
    println!("-------------------------");
}

fn print_conversation_stats(conversation: &Conversation) {
    println!("{}", conversation.name);
    println!("Teléfono: {}", conversation.phone_number);
    println!("-------------------------");
    println!("Total de mensajes: {}", conversation.message_count());
    println!("Mensajes no leidos: {}", conversation.unread_count());
    println!();
}

fn print_chats(chat: &ChatApp) {
    println!();
    println!("=== {} Conversaciones ===", chat.conversations.len());
    println!();
    println!("No leidos: {}", chat.total_unread_messages());
    println!();

    for conversation in &chat.conversations {
        print_conversation_stats(conversation);
    }
}
fn main() {
    let mut chat_app = ChatApp::new();

    chat_app.add_message(Message {
        phone_number: String::from("+50688888888"),
        sender: String::from("Juan"),
        content: String::from("Hola, Como estas?"),
        status: MessageStatus::Read,
        attachment: None,
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50688888888"),
        sender: String::from("Juan"),
        content: String::from("Muy bien, gracias!"),
        status: MessageStatus::Sent,
        attachment: Some(String::from("imagen.jpg")),
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50688888888"),
        sender: String::from("Juan"),
        content: String::from("Me alegra mucho!"),
        status: MessageStatus::Sent,
        attachment: None,
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50688888888"),
        sender: String::from("Juan"),
        content: String::from("Cuando nos vemos?"),
        status: MessageStatus::Failed,
        attachment: None,
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50689999999"),
        sender: String::from("Maria"),
        content: String::from("En donde estas?"),
        status: MessageStatus::Delivered,
        attachment: None,
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50689999999"),
        sender: String::from("Maria"),
        content: String::from("Estoy en la playa"),
        status: MessageStatus::Sent,
        attachment: None,
    });

    chat_app.add_message(Message {
        phone_number: String::from("+50689999999"),
        sender: String::from("Maria"),
        content: String::from("Vengan a acompañarme!"),
        status: MessageStatus::Sending,
        attachment: None,
    });

    print_chats(&chat_app);

    match chat_app.send_message(
        "+50688888888", 
        "Saludos, que tal!?", 
        None
    ) {
        Ok(()) => println!("Mensaje Enviado"),
        Err(error) => println!("Errror: {}", error),
    }

    print_chats(&chat_app);
}
