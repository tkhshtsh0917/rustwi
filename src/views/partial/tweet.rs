pub struct Tweet {
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl Tweet {
    pub fn new<T>(name: T, message: T, posted_at: T) -> Self
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
