use base64::{engine::general_purpose::STANDARD, Engine};

use lib0::decoding::Cursor;
use y_sync::sync::{Message, MessageReader, SyncMessage};
use yrs::updates::decoder::DecoderV1;

use super::SocketMessage;

pub fn handle_event_v1(
    socket_message: &SocketMessage,
) -> Result<(Option<Vec<u8>>, Option<Vec<u8>>, u8), lib0::error::Error> {
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
                    SyncMessage::SyncStep1(sv) => {
                        println!("SyncStep1: {:?}", sv);
                    }
                    SyncMessage::SyncStep2(update) => {
                        println!("SyncStep2: {:?}", update);
                    }
                    SyncMessage::Update(update) => {
                        println!("Update: {:?}", update);
                    }
                },
                _ => {}
            }
        }
    }
    Ok((None, None, 0))
}
