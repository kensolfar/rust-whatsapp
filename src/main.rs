#[derive(PartialEq)]
enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}
struct Message {
    sender: String,
    content: String,
    status: MessageStatus,
    attachment: Option<String>,
}

struct Conversation {
    id: u64,
    name: String,
    messages: Vec<Message>,
}

impl Conversation {
    fn new(id:u64, name: String) -> Self {
        Self {
            id,
            name,
            messages: Vec::new(),
        }
    }

    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
}

impl Message {
    fn update_status(&mut self, new_status: MessageStatus) {
        self.status = new_status;
    }

    fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }
}

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

fn print_stats(messages: &[Message]) {
    let mut leidos = 0;
    let mut no_leidos = 0;
    let mut con_adjunto = 0;

    for message in messages {
        if message.status == MessageStatus::Read {
            leidos += 1;
        } else {
            no_leidos += 1;
        }
        if message.has_attachment() {
            con_adjunto += 1;
        }
    }

    println!("Total de mensajes: {}", messages.len());
    println!("Mensajes leidos: {}", leidos);
    println!("Mensajes no leidos: {}", no_leidos);
    println!("Mensajes con adjunto: {}", con_adjunto);
    println!("-------------------------");
}
fn main() {
    let message0 = Message {
        sender: String::from("Juan"),
        content:String::from("Hola, Como estas?"),
        status: MessageStatus::Read,
        attachment: None,
    };

    let message1 = Message {
        sender: String::from("Maria"),
        content:String::from("Bien, gracias!"),
        status: MessageStatus::Delivered,
        attachment: None,
    };

    let mut messages = vec![message0, message1];

    messages.push(Message {
        sender: String::from("Pedro"),
        content: String::from("Hola a todos!"),
        status: MessageStatus::Sent,
        attachment: Some(String::from("imagen.jpg")),
    });

    for message in &messages {
        print_message(message);
    }

    /*for message in &mut messages {
        message.update_status(MessageStatus::Read);
    }*/

    print_stats(&messages);

    messages[0].update_status(MessageStatus::Failed);

    print_stats(&messages);

}   