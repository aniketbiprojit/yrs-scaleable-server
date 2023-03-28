use actix::ActorContext;
use actix_web_actors::ws::Message;

use crate::socket_message::SocketMessage;

pub(crate) struct WSActor {}

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
                handle_event.0;
            }
            _ => {}
        }
    }
}

impl actix::Actor for WSActor {
    type Context = actix_web_actors::ws::WebsocketContext<Self>;
}
