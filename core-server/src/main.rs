use actix_web::web::Data;
use config::{app_state::AppState, get_mongo_pool, parse_env};

pub mod config;
pub mod routes;
pub mod socket_message;
pub mod ws_actor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_environment = parse_env();

    let mongo_pool = get_mongo_pool(&app_environment).await;

    let app_state = Data::new(AppState::new(mongo_pool));

    return actix_web::HttpServer::new(move || {
        actix_web::App::new().app_data(app_state.clone()).route(
            "/socket.io/",
            actix_web::web::get().to(routes::socket_route),
        )
    })
    .bind(("127.0.0.1", app_environment.port))?
    .run()
    .await;
}
