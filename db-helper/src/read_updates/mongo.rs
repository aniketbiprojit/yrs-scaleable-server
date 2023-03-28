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
pub(crate) mod mongo_read_tests {
    use crate::{
        db::mongo::MongoHelper,
        read_updates::ReadUpdates,
        write_updates::{
            mongo::mongo_write_tests::{drop_collection, get_mongo_pool},
            StoreUpdate, WriteUpdates,
        },
    };

    #[tokio::test]
    async fn test_read_works() {
        let mongo_pool = get_mongo_pool().await;

        let mongo_writer = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        mongo_writer
            .write_update(&StoreUpdate {
                document_id: "test",
                update: bytes::Bytes::from(vec![1, 2, 3]),
                origin: "test_origin",
            })
            .await
            .unwrap();

        let mongo_reader = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        let updates = mongo_reader.get_updates("test").await.unwrap();

        assert_eq!(updates.len(), 1);

        drop_collection(&mongo_pool, "TestTransactionCollection").await;
    }
}
