// pub struct PostgresWriter {
//     postgres_pool: bb8::Pool<bb8_postgres::PostgresConnectionManager>>,
//     table_name: String,
// }

#[cfg(test)]
mod postgres_tests {
    use bb8::Pool;

    async fn get_postgres_pool(
    ) -> bb8::Pool<bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>> {
        let connection_manager =
            bb8_postgres::PostgresConnectionManager::new_from_stringlike("", tokio_postgres::NoTls)
                .unwrap();
        let pool = Pool::builder().build(connection_manager).await.unwrap();
        pool
    }
}
