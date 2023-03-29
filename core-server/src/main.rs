use std::collections::HashMap;
#[cfg(feature = "use_channel")]
use std::sync::mpsc::{self, Receiver, Sender};

use actix_web::web::Data;
use config::{app_state::AppState, get_mongo_pool, parse_env};
use yrs::{updates::decoder::Decode, Transact, Update};

pub mod channel;

pub mod config;
pub mod routes;
pub mod socket_message;
pub mod ws_actor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_environment = parse_env();

    let mongo_pool = get_mongo_pool(&app_environment).await;

    let app_state = Data::new(AppState::new(mongo_pool));

    #[cfg(feature = "use_channel")]
    let (tx, rx): (
        Sender<channel::UpdateMainMessage>,
        Receiver<channel::UpdateMainMessage>,
    ) = mpsc::channel();

    #[cfg(feature = "use_channel")]
    actix::spawn(async move {
        let mut docs: HashMap<String, yrs::Doc> = HashMap::new();

        for msg in rx.iter() {
            if msg.update.is_some() && (msg.message_type == 1 || msg.message_type == 2) {
                if !docs.contains_key(&msg.document_id) {
                    docs.insert(msg.document_id.clone(), yrs::Doc::new());
                }

                let mut tx = docs.get_mut(&msg.document_id).unwrap().transact_mut();

                let update = msg.update.unwrap();
                let update_to_apply = Update::decode_v1(&*update.to_vec()).unwrap();

                tx.apply_update(update_to_apply);
            }
        }
    });

    return actix_web::HttpServer::new(move || {
        #[cfg(feature = "use_channel")]
        let doc_data = Data::new(tx.clone());

        #[cfg(feature = "use_mutex")]
        let doc_data = Data::new({});

        actix_web::App::new()
            .app_data(app_state.clone())
            .app_data(doc_data)
            .route(
                "/socket.io/",
                actix_web::web::get().to(routes::socket_route),
            )
    })
    .bind(("127.0.0.1", app_environment.port))?
    .run()
    .await;
}
