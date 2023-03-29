use std::sync::mpsc::Sender;

use actix::ActorContext;
use actix_web_actors::ws::Message;

use crate::socket_message::SocketMessage;

pub(crate) struct WSActor {
    pub document_id: String,
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
                    // TODO: self.send_self("Error", "Error", ctx);
                    ctx.stop();
                    return;
                }
                let socket_message = socket_message.unwrap();

                let handle_event =
                    crate::socket_message::handle_event::handle_event_v1(&socket_message).unwrap();
                #[cfg(feature = "use_channel")]
                self.doc_data
                    .send(crate::channel::UpdateMainMessage::new(
                        &self.document_id,
                        handle_event.1,
                        handle_event.2,
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
