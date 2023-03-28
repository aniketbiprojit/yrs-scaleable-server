use bytes::Bytes;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTransactionEntity {
    pub document_id: String,
    pub origin: Option<String>,
    pub value: Option<Bytes>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
