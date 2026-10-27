use rust_whatsapp::chat_app::ChatApp;
use rust_whatsapp::display::print_chats;
use rust_whatsapp::models::message::{Message, MessageStatus};
fn main() {
    let mut chat_app = ChatApp::new();

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Hola, Como estas?"),
        MessageStatus::Read,
        None,
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Muy bien, gracias!"),
        MessageStatus::Sent,
        Some(String::from("imagen.jpg")),
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Me alegra mucho!"),
        MessageStatus::Sent,
        None,
    ));

    chat_app.add_message(Message::new(
        String::from("+50688888888"),
        String::from("Juan"),
        String::from("Cuando nos vemos?"),
        MessageStatus::Failed,
        None,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("En donde estas?"),
        MessageStatus::Delivered,
        None,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("Estoy en la playa"),
        MessageStatus::Sent,
        None,
    ));

    chat_app.add_message(Message::new(
        String::from("+50689999999"),
        String::from("Maria"),
        String::from("Vengan a acompañarme!"),
        MessageStatus::Sending,
        None,
    ));

    print_chats(&chat_app);

    match chat_app.send_message("+50688888888", "Saludos, que tal!?", None) {
        Ok(()) => println!("Mensaje Enviado"),
        Err(error) => println!("Errror: {}", error),
    }

    print_chats(&chat_app);
}
