pub mod app_state;

use std::path::Path;

use bb8::Pool;
use bb8_mongodb::MongodbConnectionManager;
use envfile::EnvFile;
use mongodb::options::ClientOptions;

pub struct AppEnvironment {
    pub port: u16,
    pub mongo_uri: String,
    pub mongo_db: String,
}

pub fn parse_env() -> AppEnvironment {
    let envfile = EnvFile::new(&Path::new(".env")).unwrap();

    let port = envfile
        .get("PORT")
        .unwrap()
        .to_string()
        .parse::<u16>()
        .unwrap();

    let mongo_uri = envfile.get("MONGO_URI").unwrap().to_string();

    let mongo_db = envfile.get("MONGO_DB_NAME").unwrap().to_string();

    let app_env = AppEnvironment {
        port,
        mongo_uri,
        mongo_db,
    };
    app_env
}

pub async fn get_mongo_pool(app_environment: &AppEnvironment) -> Pool<MongodbConnectionManager> {
    let client_options = ClientOptions::parse(app_environment.mongo_uri.clone())
        .await
        .unwrap();

    let connection_manager =
        MongodbConnectionManager::new(client_options, app_environment.mongo_db.clone());
    let pool = Pool::builder().build(connection_manager).await.unwrap();

    pool
}
