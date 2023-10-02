use std::env;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn use_connection_pool() -> ConnectionPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();
    Pool::builder().build(manager).await.unwrap()
}
