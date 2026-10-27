use rust_whatsapp::chat_app::ChatApp;
use rust_whatsapp::display::print_chats;
use rust_whatsapp::models::message::{Message, MessageStatus, MessageDirection};
fn main() {
    let mut chat_app = ChatApp::new();

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Hola, Como estas?"),
        MessageStatus::Read,
        MessageDirection::Incoming,
        None,
        false,
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Muy bien, gracias!"),
        MessageStatus::Sent,
        MessageDirection::Outgoing,
        Some(String::from("imagen.jpg")),
        false,
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Me alegra mucho!"),
        MessageStatus::Sent,
        MessageDirection::Outgoing,
        None,
        false,
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Cuando nos vemos?"),
        MessageStatus::Failed,
        MessageDirection::Outgoing,
        None,
        false,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("En donde estas?"),
        MessageStatus::Delivered,
        MessageDirection::Incoming,
        None,
        true,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("Estoy en la playa"),
        MessageStatus::Sent,
        MessageDirection::Outgoing,
        None,
        false,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("Vengan a acompañarme!"),
        MessageStatus::Sending,
        MessageDirection::Outgoing,
        None,
        false,
    ));

    print_chats(&chat_app);

    match chat_app.send_message("+50688888888", "Saludos, que tal!?", None) {
        Ok(()) => println!("Mensaje Enviado"),
        Err(error) => println!("Errror: {}", error),
    }

    print_chats(&chat_app);
}
