use crate::entities::Tweet as TweetEntity;

pub struct Tweet {
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl From<TweetEntity> for Tweet {
    fn from(entity: TweetEntity) -> Self {
        Tweet {
            name: "太郎".to_string(),
            message: entity.message,
            posted_at: entity.posted_at.format("%Y/%m/%d %H:%M").to_string(),
        }
    }
}
