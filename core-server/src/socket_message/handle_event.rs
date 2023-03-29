use base64::{engine::general_purpose::STANDARD, Engine};

use bytes::Bytes;
use lib0::decoding::Cursor;
use y_sync::sync::{Message, MessageReader, SyncMessage};
use yrs::updates::{
    decoder::DecoderV1,
    encoder::{Encode, Encoder, EncoderV1},
};

use super::SocketMessage;

pub fn handle_event_v1(
    socket_message: &SocketMessage,
) -> Result<(Option<Bytes>, Option<Bytes>, u8), lib0::error::Error> {
    {
        if &socket_message.event != "Sync" {
            return Ok((None, None, 0u8));
        }
        let decode = STANDARD.decode(&socket_message.message);

        if decode.is_err() {
            eprintln!("Error decoding message: {:?}", decode);
            return Ok((None, None, 0u8));
        }

        let message = &*decode.unwrap();
        let mut decoder = DecoderV1::new(Cursor::new(message));
        let reader = MessageReader::new(&mut decoder);
        for r in reader {
            let msg = r.unwrap();

            match msg {
                Message::Sync(msg) => match msg {
                    SyncMessage::SyncStep1(_sv) => {
                        // TODO: apply update in main
                        // println!("SyncStep1: {:?}", sv);
                    }
                    SyncMessage::SyncStep2(update) => {
                        // TODO: apply update in main
                        // println!("SyncStep2: {:?}", update);

                        let message = Message::Sync(SyncMessage::SyncStep2(update.clone()));
                        let mut encoder = EncoderV1::new();

                        message.encode(&mut encoder);
                        let message = encoder.to_vec();

                        return Ok((
                            Some(Bytes::copy_from_slice(&message)),
                            Some(Bytes::copy_from_slice(&update)),
                            1,
                        ));
                    }
                    SyncMessage::Update(update) => {
                        // TODO: apply update in main
                        // println!("Update: {:?}", update);

                        let message = Message::Sync(SyncMessage::SyncStep2(update.clone()));
                        let mut encoder = EncoderV1::new();

                        message.encode(&mut encoder);
                        let message = encoder.to_vec();
                        return Ok((
                            Some(Bytes::copy_from_slice(&message)),
                            Some(Bytes::copy_from_slice(&update)),
                            2,
                        ));
                    }
                },
                _ => {}
            }
        }
    }
    Ok((None, None, 0))
}
