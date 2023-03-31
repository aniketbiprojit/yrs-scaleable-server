#![cfg(feature = "use_channel")]

#[derive(Debug)]
pub(crate) struct UpdateMainMessage {
    pub document_id: String,
    pub update: Option<bytes::Bytes>,
    pub message_type: u8,
    pub socket_id: i32,
    pub encoded_message: String,
}

impl UpdateMainMessage {
    pub fn new(
        document_id: &str,
        update: Option<bytes::Bytes>,
        message_type: u8,
        counter: i32,
        encoded_message: String,
    ) -> Self {
        Self {
            document_id: document_id.to_string(),
            update,
            message_type,
            socket_id: counter,
            encoded_message,
        }
    }
}
