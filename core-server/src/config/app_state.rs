use bb8::Pool;
use bb8_mongodb::MongodbConnectionManager;

pub struct AppState {
    pub mongo_pool: Pool<MongodbConnectionManager>,
}

impl AppState {
    pub fn new(mongo_pool: Pool<MongodbConnectionManager>) -> Self {
        Self { mongo_pool }
    }
}
