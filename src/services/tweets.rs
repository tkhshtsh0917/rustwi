use crate::entities::Tweet;
use crate::repositories::Tweets;
use crate::views::Home;

pub async fn list_tweets<R>(repo: &R) -> Home
where
    R: Tweets,
{
    let tweets = repo.list().await;

    Home {
        tweets: tweets.into_iter().map(|tweet| tweet.into()).collect(),
    }
}

pub async fn create_tweet<R, T>(repo: &R, message: T)
where
    R: Tweets,
    T: Into<String>,
{
    let new_tweet = Tweet::create(message);
    repo.store(&new_tweet).await;
}

pub async fn delete_tweet<R>(repo: &R, id: i32)
where
    R: Tweets,
{
    let tweet = repo.find(id).await;
    if let Some(mut tweet) = tweet {
        tweet.delete();
        repo.store(&tweet).await;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::entities::Tweet;
    use crate::repositories::MockTweets;

    use super::{create_tweet, delete_tweet, list_tweets};

    fn tweet(id: i32) -> Tweet {
        Tweet::new(
            id,
            format!("message{}", id),
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
        )
    }

    #[tokio::test]
    async fn test_list_tweets() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![tweet(2), tweet(1)]);

        let actual = list_tweets(&tweets).await;
        assert_eq!(actual.tweets.len(), 2);

        let actual_0 = actual.tweets.get(0).unwrap();
        assert_eq!(actual_0.message, "message2");
        assert_eq!(actual_0.posted_at, "2020/01/01 00:00");

        let actual_1 = actual.tweets.get(1).unwrap();
        assert_eq!(actual_1.message, "message1");
        assert_eq!(actual_1.posted_at, "2020/01/01 00:00");
    }

    #[tokio::test]
    async fn test_list_tweets_empty() {
        let mut tweets = MockTweets::new();
        tweets.expect_list().returning(|| vec![]);

        let actual = list_tweets(&tweets).await;
        assert!(actual.tweets.is_empty());
    }

    #[tokio::test]
    async fn test_create_tweet() {
        let mut tweets = MockTweets::new();
        tweets
            .expect_store()
            .withf(|t| t.message == tweet(1).message)
            .once()
            .return_const(());

        let tweet = tweet(1);
        create_tweet(&tweets, &tweet.message).await;
    }

    #[tokio::test]
    async fn test_delete_tweet() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| Some(tweet(1)));
        tweets
            .expect_store()
            .withf(|t| t.id() == Some(1) && t.is_deleted())
            .once()
            .return_const(());

        delete_tweet(&tweets, 1).await;
    }

    #[tokio::test]
    async fn test_delete_tweet_not_found() {
        let mut tweets = MockTweets::new();
        tweets.expect_find().returning(|_| None);
        tweets.expect_store().never();

        delete_tweet(&tweets, 1).await;
    }
}
