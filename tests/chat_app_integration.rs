use rust_whatsapp::chat_app::{ChatApp, ChatError};
use rust_whatsapp::models::message::MessageStatus;

#[test]
fn sending_valid_message_to_existing_conversation_should_succeed() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let result = chat_app.send_message("+50683032235", "Hola, ¿cómo estás?", None);

    assert!(result.is_ok());
    let conversation = chat_app.find_conversation("+50683032235").unwrap();
    assert_eq!(conversation.message_count(), 1);
    assert_eq!(conversation.messages()[0].status(), &MessageStatus::Sending);
}

#[test]
fn sending_empty_message_should_fail() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let result = chat_app.send_message("+50683032235", "", None);

    assert!(matches!(result, Err(ChatError::EmptyMessage)));
}

#[test]
fn sending_empty_message_with_valid_attachment_should_succeed() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let result = chat_app.send_message("+50683032235", "", Some("imagen.jpg"));

    assert!(result.is_ok());
    let conversation = chat_app.find_conversation("+50683032235").unwrap();
    assert_eq!(
        conversation.messages()[0].attachment().as_deref(),
        Some("imagen.jpg")
    );
}

#[test]
fn sent_attachment_should_be_preserved_after_sending_another_message() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    chat_app
        .send_message("+50683032235", "Te envío la imagen", Some("imagen.jpg"))
        .unwrap();

    chat_app
        .send_message("+50683032235", "¿La recibiste?", None)
        .unwrap();

    let conversation = chat_app.find_conversation("+50683032235").unwrap();
    assert_eq!(
        conversation.messages()[0].attachment().as_deref(),
        Some("imagen.jpg")
    );
}

#[test]
fn sending_message_with_only_spaces_and_no_attachment_should_fail() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let result = chat_app.send_message("+50683032235", "   ", None);

    assert!(matches!(result, Err(ChatError::EmptyMessage)));
}

#[test]
fn sending_empty_message_with_attachment_name_only_spaces_should_fail() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let result = chat_app.send_message("+50683032235", "", Some("   "));

    assert!(matches!(result, Err(ChatError::EmptyMessage)));
}

#[test]
fn sending_message_to_nonexistent_conversation_should_fail() {
    let mut chat_app = ChatApp::new();
    let result = chat_app.send_message("+50683032235", "Hola, ¿cómo estás?", None);
    assert!(matches!(result, Err(ChatError::ConversationNotFound)));
}

#[test]
fn sending_valid_message_increments_message_count() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    chat_app
        .send_message("+50683032235", "Hola, ¿cómo estás?", None)
        .unwrap();

    let conversation = chat_app.find_conversation("+50683032235").unwrap();
    assert_eq!(conversation.message_count(), 1);

    chat_app
        .send_message("+50683032235", "¡Todo bien!", None)
        .unwrap();

    let conversation = chat_app.find_conversation("+50683032235").unwrap();
    assert_eq!(conversation.message_count(), 2);
}

#[test]
fn failed_operation_does_not_change_message_count() {
    let mut chat_app = ChatApp::new();
    chat_app.add_conversation(String::from("+50683032235"), String::from("Juan"));

    let initial_count = chat_app
        .find_conversation("+50683032235")
        .unwrap()
        .message_count();

    let result = chat_app.send_message("+50683032235", "", None);

    assert!(matches!(result, Err(ChatError::EmptyMessage)));
    let final_count = chat_app
        .find_conversation("+50683032235")
        .unwrap()
        .message_count();
    assert_eq!(initial_count, final_count);
}
