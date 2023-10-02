pub struct Tweet {
    pub name: String,
    pub message: String,
    pub posted_at: String,
}

impl Tweet {
    pub fn new(name: String, message: String, posted_at: String) -> Self {
        Self {
            name,
            message,
            posted_at,
        }
    }
}
