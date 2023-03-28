use mongodb::bson::DateTime;

use crate::{
    db::mongo::MongoHelper, entity::transaction_entity::BaseTransactionEntity,
    write_updates::WriteUpdates,
};

use super::StoreUpdate;

#[async_trait::async_trait]
impl<'b> WriteUpdates for MongoHelper<'b> {
    async fn write_update<'a>(&self, store_update: &'a StoreUpdate) -> Result<(), ()> {
        let db = self.mongo_pool.get().await.unwrap();
        let model = db.collection::<BaseTransactionEntity>(self.collection_name);

        let transaction_entity = BaseTransactionEntity {
            docName: Some(store_update.document_id.to_string()),
            origin: Some(store_update.origin.to_string()),
            value: Some(store_update.update.clone()),
            createdAt: Some(DateTime::now()),
            updatedAt: Some(DateTime::now()),
            isMetadataVersion: Some(false),
            name: Some(store_update.document_id.to_string()),
        };

        model.insert_one(transaction_entity, None).await.unwrap();

        return Ok(());
    }
}

#[cfg(test)]
pub(crate) mod mongo_write_tests {

    use std::time::Instant;

    use bb8::Pool;
    use bb8_mongodb::MongodbConnectionManager;
    use mongodb::{bson::doc, options::ClientOptions};

    pub(crate) async fn get_mongo_pool() -> Pool<MongodbConnectionManager> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017/")
            .await
            .unwrap();

        let connection_manager = MongodbConnectionManager::new(client_options, "TestTransactionDB");
        let mongo_pool = Pool::builder().build(connection_manager).await.unwrap();
        mongo_pool
    }

    pub(crate) async fn drop_collection(
        mongo_pool: &Pool<MongodbConnectionManager>,
        collection_name: &str,
    ) {
        let db = mongo_pool.get().await.unwrap();
        let model = db.collection::<BaseTransactionEntity>(collection_name);

        model.drop(None).await.unwrap();
    }

    pub(crate) fn get_store_updates_test_case(
        num_updates: i32,
        store_updates: &mut Vec<StoreUpdate>,
    ) {
        for _ in 0..num_updates {
            store_updates.push(StoreUpdate {
                document_id: "test",
                update: bytes::Bytes::from(vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3,
                    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6,
                    7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9,
                    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
                    12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
                    14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
                    16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                    18, 19, 20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                    20, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1,
                    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4,
                    5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 1, 2, 3, 4, 5, 6, 7,
                    8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                ]),
                origin: "test_origin",
            });
        }
    }

    use super::*;
    #[tokio::test]
    async fn test_write_works() {
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

        drop_collection(&mongo_pool, "TestTransactionCollection").await;
    }

    #[tokio::test]
    async fn multiple_writes() {
        let mongo_pool = get_mongo_pool().await;

        let collection_name = "TestTransactionCollection";
        let mongo_writer = MongoHelper::new(&mongo_pool, collection_name);

        let mut store_updates: Vec<StoreUpdate> = vec![];
        let num_updates = 10000;

        get_store_updates_test_case(num_updates, &mut store_updates);

        let start = Instant::now();

        for store_update in store_updates {
            mongo_writer.write_update(&store_update).await.unwrap();
        }

        let duration = start.elapsed();

        println!(
            "Time elapsed in multiple_writes() is: {:?} for {:?} in memory updates",
            duration, num_updates
        );
        drop_collection(&mongo_pool, collection_name).await;
    }

    #[tokio::test]
    // too expensive ignore unless you want to run it
    #[cfg_attr(not(feature = "expensive_tests"), ignore)]
    async fn multiple_writes_with_clock() {
        let collection_name = "TestTransactionCollection";

        let mongo_pool = get_mongo_pool().await;

        let mongo_writer = MongoHelper::new(&mongo_pool, collection_name);

        let mut store_updates: Vec<StoreUpdate> = vec![];
        // this should be 10_000 but it takes over 26 seconds.
        // left in 1000 for now for tests.
        let num_updates = 1000;

        get_store_updates_test_case(num_updates, &mut store_updates);

        let start = Instant::now();

        for store_update in store_updates {
            let db = mongo_pool.get().await.unwrap();
            let model = db.collection::<BaseTransactionEntity>(collection_name);

            let count = model.count_documents(doc! {}, None).await;
            if count.is_ok() {}

            mongo_writer.write_update(&store_update).await.unwrap();
        }

        let duration = start.elapsed();

        println!(
            "Time elapsed in multiple_writes_with_clock() is: {:?} for {:?} in memory updates",
            duration, num_updates
        );
        drop_collection(&mongo_pool, collection_name).await;
    }
}
