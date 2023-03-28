use bytes::Bytes;
use mongodb::bson::doc;

use super::ReadUpdates;
use futures::stream::StreamExt;

use crate::{db::mongo::MongoHelper, entity::transaction_entity::BaseTransactionEntity};

#[async_trait::async_trait]
impl<'b> ReadUpdates for MongoHelper<'b> {
    async fn get_updates(&self, document_id: &str) -> Result<Vec<Bytes>, ()> {
        let db = self.mongo_pool.get().await.unwrap();
        let model = db.collection::<BaseTransactionEntity>(self.collection_name);

        let cursor = model
            .find(doc! {"docName": document_id}, None)
            .await
            .unwrap();
        let transactions: Vec<Result<BaseTransactionEntity, mongodb::error::Error>> =
            cursor.collect().await;

        let mut updates: Vec<Bytes> = vec![];

        for transaction in transactions {
            let transaction = transaction.unwrap();
            if transaction.value.is_some() {
                updates.push(transaction.value.unwrap())
            };
        }
        return Ok(updates);
    }
}

#[cfg(test)]
pub(crate) mod mongo_read_tests {}
