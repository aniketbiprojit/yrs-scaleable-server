pub mod connect;
pub mod ws_actor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return actix_web::HttpServer::new(|| {
        actix_web::App::new().route("/ws/", actix_web::web::get().to(connect::connect))
    })
    .bind(("127.0.0.1", 2000))?
    .run()
    .await;
}
