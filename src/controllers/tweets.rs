use axum::{
    extract::{Form, Path, State},
    response::{IntoResponse, Redirect},
    routing, Router,
};
use serde::Deserialize;

use crate::{
    database::Repositories,
    repositories_impl::TweetsImpl,
    services::{create_tweet, delete_tweet},
};

#[derive(Deserialize)]
struct TweetForm {
    message: String,
}

pub fn tweets(repos: Repositories) -> Router {
    Router::new()
        .route("/new", routing::post(post_tweet_handler))
        .route("/:id/delete", routing::post(delete_tweet_handler))
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

#[tracing::instrument(skip_all)]
async fn delete_tweet_handler(
    State(repo): State<TweetsImpl>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    delete_tweet(&repo, id).await;

    Redirect::to("/")
}
