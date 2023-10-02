use std::sync::Arc;

use axum::{extract::Form, response::IntoResponse, routing, Router};
use serde::Deserialize;

use crate::{
    database::ConnectionPool,
    response,
    views::{Home, Tweet},
};

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

pub fn tweets(pool: Arc<ConnectionPool>) -> Router {
    Router::new()
        .route("/new", routing::post(post_tweet_handler))
        .with_state(pool)
}

#[tracing::instrument(skip_all)]
async fn post_tweet_handler(form: Form<TweetForm>) -> impl IntoResponse {
    let tweets = vec![Tweet::new(
        "太郎".to_string(),
        form.message.to_string(),
        "2020-01-01 12:34".to_string(),
    )];
    let home = Home::new(tweets);

    response::from_template(home)
}
