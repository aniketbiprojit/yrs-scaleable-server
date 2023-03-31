#[cfg(feature = "use_channel")]
use std::sync::mpsc::{self, Receiver, Sender};
use std::{
    sync::{Arc, Mutex},
    thread::available_parallelism,
};

use actix_web::web::Data;
use chashmap::CHashMap;
use config::{get_mongo_pool, parse_env};
use db_helper::{
    db::mongo::MongoHelper,
    write_updates::{StoreUpdate, WriteUpdates},
};
use types::BroadcastToAddresses;

#[cfg(feature = "use_channel")]
use ws_actor::Sync;

pub mod channel;

pub mod config;
pub mod routes;
pub mod socket_message;
pub mod types;
pub mod utils;
pub mod ws_actor;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_environment = parse_env();

    let mongo_pool = get_mongo_pool(&app_environment).await;

    #[cfg(feature = "use_channel")]
    let (tx, rx): (
        Sender<channel::UpdateMainMessage>,
        Receiver<channel::UpdateMainMessage>,
    ) = mpsc::channel();

    let docs: CHashMap<String, yrs::Doc> = CHashMap::new();

    let docs = Data::new(docs);

    #[cfg(feature = "use_channel")]
    let move_docs = docs.clone();

    let broadcast_to_addresses: Data<BroadcastToAddresses> = Data::new(CHashMap::new());

    // get cpu count
    let default_parallelism_approx = available_parallelism();
    let num_cpu: usize = if default_parallelism_approx.is_ok() {
        default_parallelism_approx.unwrap().get()
    } else {
        1
    };

    #[cfg(feature = "use_channel")]
    let move_broadcast_to_addresses = broadcast_to_addresses.clone();

    let updates_to_flush = Arc::new(tokio::sync::Mutex::new(Vec::<StoreUpdate>::new()));
    let move_updates_to_flush = updates_to_flush.clone();

    let mongo_writer = Arc::new(MongoHelper::new(
        mongo_pool.clone(),
        "TestTransactionCollection",
    ));

    #[cfg(feature = "use_channel")]
    actix::spawn(async move {
        let mut interval = actix::clock::interval(std::time::Duration::from_millis(100));
        loop {
            interval.tick().await;

            let store_updates;

            {
                let mut lock = updates_to_flush.lock().await;
                store_updates = lock.clone();
                lock.clear();
            }

            if store_updates.len() > 0 {
                {
                    println!("flush: {:?}", store_updates.len());
                    mongo_writer
                        .write_batch_updates(&store_updates)
                        .await
                        .unwrap();
                }
            }
        }
    });

    #[cfg(feature = "use_channel")]
    actix::spawn(async move {
        let docs = move_docs.clone();
        let mut interval = actix::clock::interval(std::time::Duration::from_millis(200));
        loop {
            interval.tick().await;
            for msg in rx.try_iter() {
                if msg.update.is_some() && (msg.message_type == 1 || msg.message_type == 2) {
                    if docs.get(&msg.document_id).is_none() {
                        docs.insert_new(msg.document_id.clone(), yrs::Doc::new());
                    }

                    let doc = docs.get_mut(&msg.document_id).unwrap();

                    let update = msg.update.unwrap();
                    utils::apply_update(&doc, &update.to_vec());

                    move_updates_to_flush.lock().await.push(StoreUpdate {
                        document_id: msg.document_id.clone(),
                        update,
                        origin: "test_origin".to_string(),
                    });

                    if move_broadcast_to_addresses.get(&msg.document_id).is_some() {
                        let broadcast_to_addresses =
                            move_broadcast_to_addresses.get(&msg.document_id).unwrap();

                        for addr in broadcast_to_addresses.iter() {
                            addr.do_send(Sync {
                                message: msg.encoded_message.to_string(),
                                event: "Sync".to_string(),
                                socket_id: msg.socket_id,
                            });
                        }
                    }
                }
            }
        }
    });

    let counter = Data::new(Mutex::new(0));

    return actix_web::HttpServer::new(move || {
        #[cfg(feature = "use_channel")]
        let doc_data = Data::new(tx.clone());

        #[cfg(feature = "use_mutex")]
        let doc_data = Data::new({});

        actix_web::App::new()
            .app_data(doc_data)
            .app_data(broadcast_to_addresses.clone())
            .app_data(counter.clone())
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
