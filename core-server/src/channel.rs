#![cfg(feature = "use_channel")]

#[derive(Debug)]
pub(crate) struct UpdateMainMessage {
    pub document_id: String,
    pub update: Option<bytes::Bytes>,
    pub message_type: u8,
}

impl UpdateMainMessage {
    pub fn new(document_id: &str, update: Option<bytes::Bytes>, message_type: u8) -> Self {
        Self {
            document_id: document_id.to_string(),
            update,
            message_type,
        }
    }
}
