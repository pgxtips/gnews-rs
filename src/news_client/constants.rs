pub const TOP_STORIES_RSS: &str = "https://news.google.com/rss";
pub const TOPICS_RSS: &str = "https://news.google.com/rss/topics/";
pub const SEARCH_RSS: &str = "https://news.google.com/rss/search?q=";

pub enum Topic {
    Technology,
    Business,
    Entertainment,
    Health,
    Science,
    Sports,
    World,
}

impl Topic {
    pub fn val(&self) -> &str {
        match self {
            Topic::Technology => "CAAqKggKIiRDQkFTRlFvSUwyMHZNRGRqTVhZU0JXVnVMVWRDR2dKSFFpZ0FQAQ",
            Topic::Business => "CAAqKggKIiRDQkFTRlFvSUwyMHZNRGx6TVdZU0JXVnVMVWRDR2dKSFFpZ0FQAQ",
            Topic::Entertainment => "CAAqKggKIiRDQkFTRlFvSUwyMHZNREpxYW5RU0JXVnVMVWRDR2dKSFFpZ0FQAQ",
            Topic::Health => "CAAqJQgKIh9DQkFTRVFvSUwyMHZNR3QwTlRFU0JXVnVMVWRDS0FBUAE",
            Topic::Science => "CAAqKggKIiRDQkFTRlFvSUwyMHZNRFp0Y1RjU0JXVnVMVWRDR2dKSFFpZ0FQAQ",
            Topic::Sports => "CAAqKggKIiRDQkFTRlFvSUwyMHZNRFp1ZEdvU0JXVnVMVWRDR2dKSFFpZ0FQAQ",
            Topic::World => "CAAqKggKIiRDQkFTRlFvSUwyMHZNRGx1YlY4U0JXVnVMVWRDR2dKSFFpZ0FQAQ",
        }
    }
}
