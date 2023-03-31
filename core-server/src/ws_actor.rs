use std::sync::Arc;

#[cfg(feature = "use_channel")]
use std::sync::mpsc::Sender;

use actix::{ActorContext, AsyncContext, Handler, Message};
use actix_web::web::Data;
use actix_web_actors::ws;
use base64::{engine::general_purpose::STANDARD, Engine};

use chashmap::CHashMap;

use crate::{
    socket_message::{handle_event, SocketMessage},
    types::BroadcastToAddresses,
};

pub(crate) struct WSActor {
    pub document_id: String,
    pub docs: Arc<CHashMap<String, yrs::Doc>>,
    #[cfg(feature = "use_channel")]
    pub doc_data: Sender<crate::channel::UpdateMainMessage>,
    pub broadcast_to_addresses: Data<BroadcastToAddresses>,
    pub socket_id: i32,
}

impl actix::StreamHandler<Result<ws::Message, actix_web_actors::ws::ProtocolError>> for WSActor {
    fn handle(
        &mut self,
        msg: Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let socket_message = serde_json::from_str::<SocketMessage>(&text);

                if socket_message.is_err() {
                    eprintln!("{:?}", socket_message.unwrap_err());
                    self.send_self("Error", "Error", ctx);
                    ctx.stop();
                    return;
                }
                let socket_message = socket_message.unwrap();

                let handle_event = handle_event::handle_event_v1(
                    &socket_message,
                    &self.document_id,
                    self.docs.clone(),
                )
                .unwrap();

                if handle_event.message.is_none() {
                    return;
                }

                let message = handle_event.message.unwrap();

                let base64_message = STANDARD.encode(message.to_vec());

                if handle_event.message_type == 0 || handle_event.message_type == 1 {
                    self.send_self("Sync", &base64_message, ctx)
                }

                // handle sync v1?
                #[cfg(feature = "use_channel")]
                self.doc_data
                    .send(crate::channel::UpdateMainMessage::new(
                        &self.document_id,
                        handle_event.update,
                        handle_event.message_type,
                        self.socket_id,
                        base64_message,
                    ))
                    .unwrap();
            }
            _ => {}
        }
    }
}

impl actix::Actor for WSActor {
    type Context = actix_web_actors::ws::WebsocketContext<Self>;

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.broadcast_to_addresses
            .get_mut(&self.document_id)
            .unwrap()
            .remove(&ctx.address());
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Sync {
    pub message: String,
    pub event: String,
    pub socket_id: i32,
}

impl Handler<Sync> for WSActor {
    type Result = ();

    fn handle(&mut self, msg: Sync, ctx: &mut Self::Context) -> Self::Result {
        if msg.socket_id == self.socket_id {
            return;
        }

        self.send_self(&msg.event, &msg.message, ctx);
    }
}

impl WSActor {
    pub fn send_self(
        &self,
        event: &str,
        message: &str,
        ctx: &mut actix_web_actors::ws::WebsocketContext<Self>,
    ) {
        ctx.text(
            serde_json::to_string(&SocketMessage {
                message: message.to_string(),
                event: event.to_string(),
            })
            .unwrap(),
        )
    }
}
