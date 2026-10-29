use rust_whatsapp::chat_app::ChatApp;
use rust_whatsapp::display::print_chats;

fn main() {
    let mut chat_app = ChatApp::new();

    chat_app
        .add_message("+50688888888", "Juan", "Hola, Como estas?", None,)
        .unwrap();

    chat_app
        .add_message("+50688888888", "Juan", "Muy bien, gracias!", Some("imagen.jpg"))
        .unwrap();

    chat_app
        .add_message("+50688888888", "Juan", "Me alegra mucho!", None)
        .unwrap();

    chat_app
        .add_message("+50688888888", "Juan", "Cuando nos vemos?", None)
        .unwrap();

    chat_app
        .add_message("+50689999999", "Maria", "En donde estas?", None)
        .unwrap();

    chat_app
        .add_message("+50689999999", "Maria", "Estoy en la playa", None)
        .unwrap();

    chat_app
        .add_message("+50689999999", "Maria", "Vengan a acompañarme!", None)
        .unwrap();

    print_chats(&chat_app);

    match chat_app.send_message("+50688888888", "Saludos, que tal!?", None) {
        Ok(()) => println!("Mensaje Enviado"),
        Err(error) => println!("Errror: {}", error),
    }

    print_chats(&chat_app);
}
