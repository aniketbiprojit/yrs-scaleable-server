pub mod mongo;

#[derive(Debug, Clone)]
pub struct StoreUpdate {
    pub document_id: String,
    pub update: bytes::Bytes,
    pub origin: String,
}

#[async_trait::async_trait]

pub trait WriteUpdates {
    async fn write_update<'a>(&self, store_update: &'a StoreUpdate) -> Result<(), ()>;

    async fn write_batch_updates<'a>(&self, store_updates: &'a Vec<StoreUpdate>) -> Result<(), ()>;
}
