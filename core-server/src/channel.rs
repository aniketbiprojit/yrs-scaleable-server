#![cfg(feature = "use_channel")]

#[derive(Debug)]
pub(crate) struct UpdateMainMessage {
    pub document_id: String,
    pub update: Option<bytes::Bytes>,
    pub message_type: u8,
    pub addr: Option<actix::Addr<crate::ws_actor::WSActor>>,
}

impl UpdateMainMessage {
    pub fn new(
        document_id: &str,
        update: Option<bytes::Bytes>,
        message_type: u8,
        addr: Option<actix::Addr<crate::ws_actor::WSActor>>,
    ) -> Self {
        Self {
            document_id: document_id.to_string(),
            update,
            message_type,
            addr,
        }
    }
}
