use std::{collections::HashMap, sync::mpsc::Sender};

use actix_web::web::{self, Query};

pub(crate) async fn socket_route(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    #[cfg(feature = "use_channel")] doc_data: web::Data<Sender<crate::channel::UpdateMainMessage>>,
    #[cfg(feature = "use_mutex")] doc_data: web::Data<()>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    let document_id = query.get("document_id").unwrap().to_string();
    let resp = actix_web_actors::ws::start(
        crate::ws_actor::WSActor {
            document_id,
            #[cfg(feature = "use_channel")]
            doc_data: doc_data.get_ref().clone(),
            #[cfg(feature = "use_mutex")]
            doc_data: (),
        },
        &req,
        stream,
    );
    resp
}
