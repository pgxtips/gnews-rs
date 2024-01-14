use serde::Serialize;

#[derive(Serialize)]
pub struct News {
    pub title: String,
    pub rss_link: String,
    pub origin_link: String,
    pub description: String,
    pub source: String,
    pub pub_date: String,
    pub guid: String,
    pub comments: String,
}
impl News {
    pub fn new() -> News {
        News {
            title: String::new(),
            rss_link: String::new(),
            origin_link: String::new(),
            description: String::new(),
            source: String::new(),
            pub_date: String::new(),
            guid: String::new(),
            comments: String::new()
        }
    }
}
