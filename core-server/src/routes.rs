use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[cfg(feature = "use_channel")]
use std::sync::mpsc::Sender;

use actix_web::web::{self, Query};
use actix_web_actors::ws::WsResponseBuilder;

use crate::types::BroadcastToAddresses;

pub(crate) async fn socket_route(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    docs: web::Data<crate::CHashMap<String, yrs::Doc>>,
    broadcast_to_addresses: web::Data<BroadcastToAddresses>,
    #[cfg(feature = "use_channel")] doc_data: web::Data<Sender<crate::channel::UpdateMainMessage>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    let docs = docs.into_inner();

    let document_id = query.get("document_id").unwrap().to_string();

    if docs.get(&document_id).is_none() {
        docs.insert_new(document_id.clone(), yrs::Doc::new());
    }

    let (addr, resp) = WsResponseBuilder::new(
        crate::ws_actor::WSActor {
            document_id: document_id.clone(),
            docs: Arc::clone(&docs),
            #[cfg(feature = "use_channel")]
            doc_data: doc_data.get_ref().clone(),
            broadcast_to_addresses: broadcast_to_addresses.clone(),
        },
        &req,
        stream,
    )
    .start_with_addr()
    .unwrap();

    if broadcast_to_addresses.get(&document_id.clone()).is_none() {
        broadcast_to_addresses.insert_new(document_id.to_string(), HashSet::new());
    }

    broadcast_to_addresses
        .get_mut(&document_id)
        .unwrap()
        .insert(addr.clone());

    Ok(resp)
}
