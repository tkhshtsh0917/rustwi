use chrono::{DateTime, Utc};

pub struct Tweet {
    #[allow(dead_code)]
    id: Option<i32>,
    pub message: String,
    pub posted_at: DateTime<Utc>,
}

impl Tweet {
    pub fn new(id: i32, message: String, posted_at: DateTime<Utc>) -> Self {
        Self {
            id: Some(id),
            message,
            posted_at,
        }
    }

    pub fn create<T>(message: T) -> Self
    where
        T: Into<String>,
    {
        Tweet {
            id: None,
            message: message.into(),
            posted_at: Utc::now(),
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Option<i32> {
        self.id
    }
}
