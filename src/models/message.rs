#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum DeliveryStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}

#[derive(PartialEq)]
pub struct Message {
    phone_number: String,
    sender: String,
    content: String,
    status: Option<DeliveryStatus>,
    direction: MessageDirection,
    attachment: Option<String>,
    is_read: bool,
}

impl Message {
    /// Crea un mensaje.
    ///
    /// Los mensajes entrantes siempre se crean sin estado de entrega. El
    /// estado proporcionado solo se conserva para mensajes salientes.
    pub fn new(
        phone_number: String,
        sender: String,
        content: String,
        status: Option<DeliveryStatus>,
        direction: MessageDirection,
        attachment: Option<String>,
        is_read: bool,
    ) -> Self {
        let status = match direction {
            MessageDirection::Incoming => None,
            MessageDirection::Outgoing => status,
        };

        Self {
            phone_number,
            sender,
            content,
            status,
            direction,
            attachment,
            is_read,
        }
    }
    /// Devuelve el remitente del mensaje.
    pub fn sender(&self) -> &str {
        &self.sender
    }

    /// Devuelve el contenido textual del mensaje.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Devuelve el número telefónico asociado al mensaje.
    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    /// Devuelve el estado de entrega del mensaje, si tiene uno.
    pub fn status(&self) -> Option<&DeliveryStatus> {
        self.status.as_ref()
    }

    /// Devuelve la dirección del mensaje.
    pub fn direction(&self) -> &MessageDirection {
        &self.direction
    }

    /// Devuelve el nombre del archivo adjunto, si existe.
    pub fn attachment(&self) -> Option<&str> {
        self.attachment.as_deref()
    }

    /// Actualiza el estado de entrega de un mensaje saliente.
    ///
    /// Los mensajes entrantes no modifican su estado y permanecen sin estado
    /// de entrega.
    pub fn update_status(&mut self, new_status: DeliveryStatus) {
        if self.direction != MessageDirection::Incoming {
            self.status = Some(new_status);
        }
    }

    /// Indica si el mensaje tiene un archivo adjunto.
    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    /// Indica si el mensaje fue marcado como leído.
    pub fn is_read(&self) -> bool {
        self.is_read
    }

    /// Marca el mensaje como leído.
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(attachment: Option<&str>) -> Message {
        Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            None,
            MessageDirection::Incoming,
            attachment.map(String::from),
            false,
        )
    }

    #[test]
    fn new_message_no_attachment_should_not_have_attachment() {
        let message = message(None);

        assert!(!message.has_attachment());
    }

    #[test]
    fn new_message_with_attachment_should_have_attachment() {
        let message = message(Some("image.png"));
        assert!(message.has_attachment());
    }

    #[test]
    fn new_message_should_have_correct_status() {
        let message = message(None);

        assert_eq!(message.status, None);
    }

    #[test]
    fn new_incoming_message_should_ignore_provided_status() {
        let message = Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            Some(DeliveryStatus::Sending),
            MessageDirection::Incoming,
            None,
            false,
        );

        assert_eq!(message.status(), None);
    }

    #[test]
    fn update_status_should_change_message_status() {
        let mut message = Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            Some(DeliveryStatus::Sending),
            MessageDirection::Outgoing,
            None,
            false,
        );
        message.update_status(DeliveryStatus::Sent);
        assert_eq!(message.status, Some(DeliveryStatus::Sent));
    }

    #[test]
    fn update_status_should_not_change_an_incoming_message_status() {
        let mut message = Message::new(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            None,
            MessageDirection::Incoming,
            None,
            false,
        );

        message.update_status(DeliveryStatus::Read);

        assert_eq!(message.status(), None);
    }
}
