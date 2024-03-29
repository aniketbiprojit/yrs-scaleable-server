use bytes::Bytes;

pub mod mongo;

/// This trait is used to read updates from the database.
/// This should be enough to be consumed by servers to get updates.
#[async_trait::async_trait]
pub trait ReadUpdates {
    async fn get_updates(&self, document_id: &str) -> Result<Vec<Bytes>, ()>;
}
