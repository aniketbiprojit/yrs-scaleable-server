pub(crate) struct WSActor {}

impl
    actix::StreamHandler<Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>>
    for WSActor
{
    fn handle(
        &mut self,
        msg: Result<actix_web_actors::ws::Message, actix_web_actors::ws::ProtocolError>,
        _ctx: &mut Self::Context,
    ) {
        match msg {
            _ => (),
        }
    }
}

impl actix::Actor for WSActor {
    type Context = actix_web_actors::ws::WebsocketContext<Self>;
}
