use axum::{extract::State, response::IntoResponse, routing, Router};

use crate::{
    controllers::tweets, database, repositories_impl::TweetsImpl, response, services::list_tweets,
};

pub async fn app() -> Router {
    let repos = database::resolve_repositories().await;

    Router::new()
        .route("/", routing::get(home_view_handler))
        .with_state(repos.clone())
        .nest("/tweets", tweets(repos.clone()))
}

#[tracing::instrument(skip_all)]
async fn home_view_handler(State(repo): State<TweetsImpl>) -> impl IntoResponse {
    let home = list_tweets(&repo).await;

    response::from_template(home)
}
