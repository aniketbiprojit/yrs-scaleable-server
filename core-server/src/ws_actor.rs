use std::sync::{mpsc::Sender, Arc};

use actix::ActorContext;
use actix_web_actors::ws::Message;
use base64::{engine::general_purpose::STANDARD, Engine};

use chashmap::CHashMap;

use crate::socket_message::{handle_event, SocketMessage};

pub(crate) struct WSActor {
    pub document_id: String,
    pub docs: Arc<CHashMap<String, yrs::Doc>>,
    #[cfg(feature = "use_mutex")]
    pub doc_data: (),
    #[cfg(feature = "use_channel")]
    pub doc_data: Sender<crate::channel::UpdateMainMessage>,
}

impl
    actix::StreamHandler<Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>>
    for WSActor
{
    fn handle(
        &mut self,
        msg: Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(Message::Text(text)) => {
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

                if handle_event.message_type == 0 || handle_event.message_type == 1 {
                    self.send_self("Sync", &STANDARD.encode(message.to_vec()), ctx)
                }

                // handle sync v1?
                #[cfg(feature = "use_channel")]
                self.doc_data
                    .send(crate::channel::UpdateMainMessage::new(
                        &self.document_id,
                        handle_event.update,
                        handle_event.message_type,
                    ))
                    .unwrap();
            }
            _ => {}
        }
    }
}

impl actix::Actor for WSActor {
    type Context = actix_web_actors::ws::WebsocketContext<Self>;
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
