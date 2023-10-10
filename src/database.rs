use std::{env, sync::Arc};

use axum::extract::FromRef;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::repositories_impl::TweetsImpl;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(FromRef, Clone)]
pub struct Repositories {
    tweets: TweetsImpl,
}

pub async fn resolve_repositories() -> Repositories {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls).unwrap();

    let pool = Arc::new(Pool::builder().build(manager).await.unwrap());

    Repositories {
        tweets: TweetsImpl { pool: pool.clone() },
    }
}
