use axum::{response::IntoResponse, routing, Router};

use crate::response;
use crate::views::{Home, Tweet};

pub fn app() -> Router {
    Router::new().route("/", routing::get(get_handler))
}

async fn get_handler() -> impl IntoResponse {
    let tweets = (1..=20)
        .into_iter()
        .map(|_| Tweet::new("太郎", "こんにちは！", "2020-01-01 12:34"))
        .collect();

    let home = Home::new(tweets);

    response::from_template(home)
}
