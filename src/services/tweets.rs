use crate::repositories::Tweets;
use crate::views::Home;

pub async fn list_tweets<T>(repo: &T) -> Home
where
    T: Tweets,
{
    let tweets = repo.list().await;

    Home {
        tweets: tweets.into_iter().map(|tweet| tweet.into()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::entities::Tweet;
    use crate::repositories::MockTweets;

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

        let actual = super::list_tweets(&tweets).await;
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

        let actual = super::list_tweets(&tweets).await;
        assert!(actual.tweets.is_empty());
    }
}
