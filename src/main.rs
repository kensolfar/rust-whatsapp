use rust_whatsapp::chat_app::ChatApp;
use rust_whatsapp::display::print_chats;
use rust_whatsapp::models::message::{Message, MessageStatus};
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
