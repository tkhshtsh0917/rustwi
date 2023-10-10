use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::{database::Repositories, repositories_impl::TweetsImpl, services::create_tweet};

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

pub fn tweets(repos: Repositories) -> Router {
    Router::new()
        .route("/new", routing::post(post_tweet_handler))
        .with_state(repos)
}

#[tracing::instrument(skip_all)]
async fn post_tweet_handler(
    State(repo): State<TweetsImpl>,
    form: Form<TweetForm>,
) -> impl IntoResponse {
    create_tweet(&repo, &form.message).await;

    Redirect::to("/")
}
