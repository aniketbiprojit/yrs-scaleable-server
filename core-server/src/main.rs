#[cfg(feature = "use_channel")]
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::available_parallelism;

use actix_web::web::Data;
use chashmap::CHashMap;
use config::{app_state::AppState, get_mongo_pool, parse_env};

pub mod channel;

pub mod config;
pub mod routes;
pub mod socket_message;
pub mod utils;
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
    let docs: CHashMap<String, yrs::Doc> = CHashMap::new();

    let docs = Data::new(docs);

    #[cfg(feature = "use_channel")]
    let move_docs = docs.clone();

    // get cpu count
    let default_parallelism_approx = available_parallelism();
    let num_cpu: usize = if default_parallelism_approx.is_ok() {
        default_parallelism_approx.unwrap().get()
    } else {
        1
    };

    #[cfg(feature = "use_channel")]
    actix::spawn(async move {
        let docs = move_docs.clone();
        for msg in rx.iter() {
            if msg.update.is_some() && (msg.message_type == 1 || msg.message_type == 2) {
                if docs.get(&msg.document_id).is_none() {
                    docs.insert_new(msg.document_id.clone(), yrs::Doc::new());
                }

                let doc = docs.get_mut(&msg.document_id).unwrap();

                utils::apply_update(&doc, &msg.update.unwrap().to_vec());
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
            .app_data(docs.clone())
            .route(
                "/socket.io/",
                actix_web::web::get().to(routes::socket_route),
            )
    })
    .workers(num_cpu * 2)
    .bind(("127.0.0.1", app_environment.port))?
    .run()
    .await;
}
