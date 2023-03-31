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
                if transaction.isMetadataVersion.is_none()
                    || (transaction.isMetadataVersion.is_some()
                        && transaction.isMetadataVersion.unwrap() == false)
                {
                    updates.push(transaction.value.unwrap())
                }
            };
        }
        return Ok(updates);
    }
}

#[cfg(test)]
pub(crate) mod mongo_read_tests {
    use std::time::Instant;

    use crate::{
        db::mongo::MongoHelper,
        read_updates::ReadUpdates,
        write_updates::{
            mongo::mongo_write_tests::{
                drop_collection, get_mongo_pool, get_store_updates_test_case,
            },
            StoreUpdate, WriteUpdates,
        },
    };

    #[tokio::test]
    async fn test_read_works() {
        let mongo_pool = get_mongo_pool().await;

        let mongo_writer = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        mongo_writer
            .write_update(&StoreUpdate {
                document_id: "test".to_string(),
                update: bytes::Bytes::from(vec![1, 2, 3]),
                origin: "test_origin".to_string(),
            })
            .await
            .unwrap();

        let mongo_reader = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        let updates = mongo_reader.get_updates("test").await.unwrap();

        assert_eq!(updates.len(), 1);

        drop_collection(&mongo_pool, "TestTransactionCollection").await;
    }

    #[tokio::test]
    async fn test_multiple_reads() {
        let mongo_pool = get_mongo_pool().await;

        let mongo_writer = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        let collection_name = "TestTransactionCollection";

        let mut store_updates: Vec<StoreUpdate> = vec![];

        let num_updates = 10_000;
        drop_collection(&mongo_pool, collection_name).await;

        get_store_updates_test_case(num_updates, &mut store_updates);

        for store_update in store_updates {
            mongo_writer.write_update(&store_update).await.unwrap();
        }

        let start = Instant::now();

        let mongo_reader = MongoHelper::new(&mongo_pool, "TestTransactionCollection");

        let updates = mongo_reader.get_updates("test").await.unwrap();

        assert_eq!(updates.len(), num_updates as usize);

        let duration = start.elapsed();

        println!(
            "Time in test_multiple_reads() is: {:?} for {:?} in memory updates",
            duration, num_updates
        );
        drop_collection(&mongo_pool, collection_name).await;
    }
}
