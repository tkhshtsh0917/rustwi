use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Template)]
#[template(path = "home.html")]
struct Home {
    tweets: Vec<Tweet>,
}

struct Tweet {
    name: String,
    message: String,
    posted_at: String,
}

impl Tweet {
    fn new<T>(name: T, message: T, posted_at: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            message: message.into(),
            posted_at: posted_at.into(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustwi=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    let tweets = (1..=20)
        .into_iter()
        .map(|_| Tweet::new("太郎", "こんにちわ！", "2020-01-01 12:34"))
        .collect();

    let home = Home { tweets };

    Html(home.render().unwrap()).into_response()
}
