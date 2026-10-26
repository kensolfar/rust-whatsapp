#[derive(PartialEq, Debug, Clone, Eq, Hash)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}
#[derive(PartialEq)]
pub struct Message {
    pub phone_number: String,
    pub sender: String,
    pub content: String,
    pub status: MessageStatus,
    pub attachment: Option<String>,
}

impl Message {
    pub fn update_status(&mut self, new_status: MessageStatus) {
        self.status = new_status;
    }

    pub fn has_attachment(&self) -> bool {
        self.attachment.is_some()
    }
}
