use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "use_channel")]
use std::sync::mpsc::Sender;

use actix_web::web::{self, Query};

pub(crate) async fn socket_route(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    docs: web::Data<crate::CHashMap<String, yrs::Doc>>,
    #[cfg(feature = "use_channel")] doc_data: web::Data<Sender<crate::channel::UpdateMainMessage>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    let docs = docs.into_inner();

    let document_id = query.get("document_id").unwrap().to_string();

    if docs.get(&document_id).is_none() {
        docs.insert_new(document_id.clone(), yrs::Doc::new());
    }

    let resp = actix_web_actors::ws::start(
        crate::ws_actor::WSActor {
            document_id,
            docs: Arc::clone(&docs),
            #[cfg(feature = "use_channel")]
            doc_data: doc_data.get_ref().clone(),
        },
        &req,
        stream,
    );
    resp
}
