use std::sync::Arc;

use base64::{engine::general_purpose::STANDARD, Engine};

use bytes::Bytes;
use chashmap::CHashMap;
use lib0::decoding::Cursor;
use y_sync::sync::{Message, MessageReader, SyncMessage};
use yrs::{
    updates::{
        decoder::DecoderV1,
        encoder::{Encode, Encoder, EncoderV1},
    },
    ReadTxn, Transact,
};

#[cfg(feature = "use_mutex")]
use crate::utils::apply_update;

use super::SocketMessage;

#[derive(Default, Debug)]
pub struct HandleEvent {
    pub message: Option<Bytes>,
    pub update: Option<Bytes>,
    pub message_type: u8,
}

impl HandleEvent {
    fn new(message: Vec<u8>, update: Vec<u8>, message_type: u8) -> Self {
        Self {
            message: Some(Bytes::copy_from_slice(&message)),
            update: Some(Bytes::copy_from_slice(&update)),
            message_type,
        }
    }
}

pub fn handle_event_v1(
    socket_message: &SocketMessage,
    document_id: &str,
    docs: Arc<CHashMap<String, yrs::Doc>>,
) -> Result<HandleEvent, lib0::error::Error> {
    {
        if &socket_message.event != "Sync" {
            return Ok(HandleEvent::default());
        }
        let decode = STANDARD.decode(&socket_message.message);

        if decode.is_err() {
            eprintln!("Error decoding message: {:?}", decode);
            return Ok(HandleEvent::default());
        }

        let message = &*decode.unwrap();
        let mut decoder = DecoderV1::new(Cursor::new(message));
        let reader = MessageReader::new(&mut decoder);
        for r in reader {
            let msg = r.unwrap();

            match msg {
                Message::Sync(msg) => match msg {
                    SyncMessage::SyncStep1(sv) => {
                        let doc = docs.get(document_id).unwrap();
                        let update = doc.transact().encode_state_as_update_v1(&sv);
                        let mut encoder = EncoderV1::new();
                        let message = Message::Sync(SyncMessage::SyncStep2(update.clone()));
                        message.encode(&mut encoder);
                        let message = encoder.to_vec();
                        return Ok(HandleEvent::new(message, update, 0));
                    }
                    SyncMessage::SyncStep2(update) => {
                        #[cfg(feature = "use_mutex")]
                        {
                            let doc = docs.get_mut(document_id).unwrap();
                            apply_update(&doc, &update);
                        }
                        let message = Message::Sync(SyncMessage::SyncStep2(update.clone()));
                        let mut encoder = EncoderV1::new();

                        message.encode(&mut encoder);
                        let message = encoder.to_vec();
                        // broadcast
                        return Ok(HandleEvent::new(message, update, 1));
                    }
                    SyncMessage::Update(update) => {
                        #[cfg(feature = "use_mutex")]
                        {
                            let doc = docs.get_mut(document_id).unwrap();
                            apply_update(&doc, &update);
                        }
                        let message = Message::Sync(SyncMessage::SyncStep2(update.clone()));
                        let mut encoder = EncoderV1::new();

                        message.encode(&mut encoder);
                        // broadcast
                        let message = encoder.to_vec();
                        return Ok(HandleEvent::new(message, update, 2));
                    }
                },
                _ => {}
            }
        }
    }
    Ok(HandleEvent::default())
}
