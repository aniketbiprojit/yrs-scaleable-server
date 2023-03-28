use bytes::Bytes;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTransactionEntity {
    pub docName: Option<String>,
    pub origin: Option<String>,
    pub value: Option<Bytes>,
    pub createdAt: Option<DateTime>,
    pub updatedAt: Option<DateTime>,
    pub isMetadataVersion: Option<bool>,
    pub name: Option<String>,
}
