use std::sync::Arc;

use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Tweet;
use crate::repositories::Tweets;

const FIND_QUERY: &str = "SELECT * FROM tweets WHERE id = $1";
const LIST_QUERY: &str = "SELECT * FROM tweets ORDER BY posted_at DESC";
const DELETE_QUERY: &str = "DELETE FROM tweets WHERE id = $1";
const INSERT_QUERY: &str = "INSERT INTO tweets (message, posted_at) VALUES ($1, $2)";

impl From<Row> for Tweet {
    fn from(r: Row) -> Self {
        Tweet::new(r.get("id"), r.get("message"), r.get("posted_at"))
    }
}

#[derive(Clone)]
pub struct TweetsImpl {
    pub pool: Arc<ConnectionPool>,
}

#[axum::async_trait]
impl Tweets for TweetsImpl {
    async fn find(&self, id: i32) -> Option<Tweet> {
        let conn = self.pool.get().await.unwrap();

        let row = conn.query_opt(FIND_QUERY, &[&id]).await.unwrap();

        row.map(|r| r.into())
    }

    async fn list(&self) -> Vec<Tweet> {
        let conn = self.pool.get().await.unwrap();

        let rows = conn.query(LIST_QUERY, &[]).await.unwrap();

        rows.into_iter().map(|r| r.into()).collect()
    }

    async fn store(&self, entity: &Tweet) {
        let conn = self.pool.get().await.unwrap();

        if let Some(id) = entity.id() {
            if entity.is_deleted() {
                conn.execute(DELETE_QUERY, &[&id]).await.ok();
            }
        } else {
            conn.execute(INSERT_QUERY, &[&entity.message, &entity.posted_at])
                .await
                .ok();
        }
    }
}
