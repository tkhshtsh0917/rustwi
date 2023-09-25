use askama::Template;

use crate::views::partial::Tweet;

#[derive(Template)]
#[template(path = "home.html")]
pub struct Home {
    pub tweets: Vec<Tweet>,
}

impl Home {
    pub fn new(tweets: Vec<Tweet>) -> Self {
        Self { tweets }
    }
}
