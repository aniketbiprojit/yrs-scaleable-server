pub mod mongo;

#[derive(Debug)]
pub struct StoreUpdate<'a> {
    pub document_id: &'a str,
    pub update: bytes::Bytes,
    pub origin: &'a str,
}

#[async_trait::async_trait]

pub trait WriteUpdates {
    async fn write_update<'a>(&self, store_update: &'a StoreUpdate) -> Result<(), ()>;
}
