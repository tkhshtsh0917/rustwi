mod controllers {
    mod root;
    mod tweets;

    pub use root::app;
    pub(crate) use tweets::tweets;
}

mod entities {
    mod tweet;

    pub(crate) use tweet::Tweet;
}

mod repositories {
    mod tweets;

    pub(crate) use tweets::Tweets;
}

mod repositories_impl {
    mod tweets;

    pub(crate) use tweets::TweetsImpl;
}

mod services {
    mod tweets;

    pub(crate) use tweets::list_tweets;
}

mod views {
    mod home;
    mod partial {
        mod tweet;

        pub(crate) use tweet::Tweet;
    }

    pub(crate) use home::Home;
    pub(crate) use partial::Tweet;
}

mod database;
mod response;

pub use controllers::app;
