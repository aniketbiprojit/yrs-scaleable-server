pub(crate) async fn socket_route(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let resp = actix_web_actors::ws::start(crate::ws_actor::WSActor {}, &req, stream);
    resp
}
