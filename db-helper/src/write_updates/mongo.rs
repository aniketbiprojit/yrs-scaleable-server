use mongodb::bson::DateTime;

use crate::{entity::transaction_entity::BaseTransactionEntity, write_updates::WriteUpdates};

use super::StoreUpdate;

pub struct MongoWriter {
    mongo_pool: bb8::Pool<bb8_mongodb::MongodbConnectionManager>,
    collection_name: String,
}

impl MongoWriter {
    pub fn new(
        mongo_pool: bb8::Pool<bb8_mongodb::MongodbConnectionManager>,
        collection_name: String,
    ) -> Self {
        Self {
            mongo_pool,
            collection_name,
        }
    }
}

#[async_trait::async_trait]
impl WriteUpdates for MongoWriter {
    async fn write_update<'a>(&self, store_update: &'a StoreUpdate) -> Result<(), ()> {
        let db = self.mongo_pool.get().await.unwrap();
        let model = db.collection::<BaseTransactionEntity>(self.collection_name.as_str());

        let transaction_entity = BaseTransactionEntity {
            document_id: store_update.document_id.to_string(),
            origin: Some(store_update.origin.to_string()),
            value: Some(store_update.update.clone()),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        };

        model.insert_one(transaction_entity, None).await.unwrap();

        return Ok(());
    }
}

#[cfg(test)]
mod mongo_tests {

    use bb8::Pool;
    use bb8_mongodb::MongodbConnectionManager;
    use mongodb::options::ClientOptions;

    async fn get_mongo_pool() -> Pool<MongodbConnectionManager> {
        let client_options = ClientOptions::parse("mongodb://localhost:27017/")
            .await
            .unwrap();

        let connection_manager = MongodbConnectionManager::new(client_options, "TestTransactionDB");
        let mongo_pool = Pool::builder().build(connection_manager).await.unwrap();
        mongo_pool
    }

    use super::*;
    #[tokio::test]
    async fn test_write_works() {
        let mongo_pool = get_mongo_pool().await;

        let mongo_writer = MongoWriter::new(mongo_pool, "TestTransactionCollection".to_string());

        mongo_writer
            .write_update(&StoreUpdate {
                document_id: "test",
                update: bytes::Bytes::from(vec![1, 2, 3]),
                origin: "test_origin",
            })
            .await
            .unwrap();

        println!("Done!");
    }
}
