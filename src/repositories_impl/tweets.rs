use std::sync::Arc;

use crate::database::ConnectionPool;
use crate::entities::Tweet;
use crate::repositories::Tweets;

const SELECT_QUERY: &str = "SELECT * FROM tweets ORDER BY posted_at DESC";
const INSERT_QUERY: &str = "INSERT INTO tweets (message, posted_at) VALUES ($1, $2)";

#[derive(Clone)]
pub struct TweetsImpl {
    pub pool: Arc<ConnectionPool>,
}

#[axum::async_trait]
impl Tweets for TweetsImpl {
    async fn list(&self) -> Vec<Tweet> {
        let conn = self.pool.get().await.unwrap();

        let rows = conn.query(SELECT_QUERY, &[]).await.unwrap();

        rows.into_iter()
            .map(|r| Tweet::new(r.get("id"), r.get("message"), r.get("posted_at")))
            .collect()
    }

    async fn store(&self, entity: &Tweet) {
        let conn = self.pool.get().await.unwrap();

        conn.execute(INSERT_QUERY, &[&entity.message, &entity.posted_at])
            .await
            .ok();
    }
}
