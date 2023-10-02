use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing, Router};
use chrono::{DateTime, Utc};

use crate::{
    controllers::tweets,
    database::{self, ConnectionPool},
    response,
    views::{Home, Tweet},
};

pub async fn app() -> Router {
    let connection_pool = Arc::new(database::use_connection_pool().await);

    Router::new()
        .route("/", routing::get(home_view_handler))
        .with_state(connection_pool.clone())
        .nest("/tweets", tweets(connection_pool))
}

#[tracing::instrument]
async fn home_view_handler(State(pool): State<Arc<ConnectionPool>>) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();
    let rows = conn
        .query("SELECT * FROM tweets ORDER BY posted_at DESC", &[])
        .await
        .unwrap();

    let tweets = rows
        .into_iter()
        .map(|row| {
            Tweet::new(
                "太郎".to_string(),
                row.get("message"),
                row.get::<&str, DateTime<Utc>>("posted_at")
                    .format("%Y/%m/%d %H:%M")
                    .to_string(),
            )
        })
        .collect();

    let home = Home::new(tweets);

    response::from_template(home)
}
