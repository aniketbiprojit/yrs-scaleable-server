pub struct MongoHelper<'a> {
    pub mongo_pool: &'a bb8::Pool<bb8_mongodb::MongodbConnectionManager>,
    pub collection_name: &'a str,
}

impl<'a> MongoHelper<'a> {
    pub fn new(
        mongo_pool: &'a bb8::Pool<bb8_mongodb::MongodbConnectionManager>,
        collection_name: &'a str,
    ) -> Self {
        Self {
            mongo_pool,
            collection_name,
        }
    }
}
