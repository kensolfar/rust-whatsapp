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

#[derive(Debug, PartialEq, Eq)]
pub enum MessageKind {
    Incoming {
        is_read: bool,
    },
    Outgoing {
        delivery_status: DeliveryStatus,
    },
}

#[derive(PartialEq)]
pub struct Message {
    phone_number: String,
    sender: String,
    content: String,
    kind: MessageKind,
    attachment: Option<String>,
}

impl Message {
    /// Crea un mensaje entrante sin estado de entrega.
    pub fn new_incoming(
        phone_number: String,
        sender: String,
        content: String,
        attachment: Option<String>,
        is_read: bool,
    ) -> Self {
        Self {
            phone_number,
            sender,
            content,
            kind: MessageKind::Incoming { is_read },
            attachment,
        }
    }

    /// Crea un mensaje saliente con su estado de entrega inicial.
    pub fn new_outgoing(
        phone_number: String,
        sender: String,
        content: String,
        delivery_status: DeliveryStatus,
        attachment: Option<String>,
    ) -> Self {
        Self {
            phone_number,
            sender,
            content,
            kind: MessageKind::Outgoing { delivery_status },
            attachment,
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
    pub fn delivery_status(&self) -> Option<&DeliveryStatus> {
        match &self.kind {
            MessageKind::Incoming { .. } => None,
            MessageKind::Outgoing { delivery_status } => Some(delivery_status),
        }
    }

    /// Devuelve la dirección del mensaje.
    pub fn direction(&self) -> MessageDirection {
        match self.kind {
            MessageKind::Incoming { .. } => MessageDirection::Incoming,
            MessageKind::Outgoing { .. } => MessageDirection::Outgoing,
        }
    }

    pub fn kind(&self) -> &MessageKind {
        &self.kind
    }

    /// Devuelve el nombre del archivo adjunto, si existe.
    pub fn attachment(&self) -> Option<&str> {
        self.attachment.as_deref()
    }

    /// Actualiza el estado de entrega de un mensaje saliente.
    ///
    /// Los mensajes entrantes no modifican su estado y permanecen sin estado
    /// de entrega.
    pub fn update_delivery_status(&mut self, new_status: DeliveryStatus) {
        if let MessageKind::Outgoing { delivery_status } = &mut self.kind {
            *delivery_status = new_status;
        }
    }

    /// Indica si el mensaje tiene un archivo adjunto.
    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }

    /// Indica si el mensaje fue marcado como leído.
    pub fn is_read(&self) -> bool {
        match &self.kind {
            MessageKind::Incoming { is_read } => *is_read,
            MessageKind::Outgoing { .. } => true,
        }
    }

    /// Marca el mensaje como leído.
    pub fn mark_as_read(&mut self) {
        if let MessageKind::Incoming { is_read } = &mut self.kind {
            *is_read = true;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(attachment: Option<&str>) -> Message {
        Message::new_incoming(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
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

        assert_eq!(message.delivery_status(), None);
    }

    #[test]
    fn new_incoming_message_should_have_no_delivery_status() {
        let message = Message::new_incoming(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            None,
            false,
        );

        assert_eq!(message.delivery_status(), None);
    }

    #[test]
    fn update_status_should_change_message_status() {
        let mut message = Message::new_outgoing(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            DeliveryStatus::Sending,
            None,
        );
        message.update_delivery_status(DeliveryStatus::Sent);
        assert_eq!(message.delivery_status(), Some(&DeliveryStatus::Sent));
    }

    #[test]
    fn update_status_should_not_change_an_incoming_message_status() {
        let mut message = Message::new_incoming(
            String::from("+50683032235"),
            String::from("Juan"),
            String::from("Hola, ¿cómo estás?"),
            None,
            false,
        );

        message.update_delivery_status(DeliveryStatus::Read);

        assert_eq!(message.delivery_status(), None);
    }
}
