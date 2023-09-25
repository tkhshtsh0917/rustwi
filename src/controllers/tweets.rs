use axum::{extract::Form, response::IntoResponse, routing, Router};
use serde::Deserialize;

use crate::response;
use crate::views::{Home, Tweet};

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

pub fn tweets() -> Router {
    Router::new().route("/new", routing::post(post_tweet_handler))
}

#[tracing::instrument(skip_all)]
async fn post_tweet_handler(form: Form<TweetForm>) -> impl IntoResponse {
    let tweets = vec![Tweet::new("太郎", &form.message, "2020-01-01 12:34")];
    let home = Home::new(tweets);

    response::from_template(home)
}
