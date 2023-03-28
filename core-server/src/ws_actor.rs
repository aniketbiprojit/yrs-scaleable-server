use actix_web_actors::ws;

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
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl actix::Actor for WSActor {
    type Context = actix_web_actors::ws::WebsocketContext<Self>;
}
