use serde::Serialize;

#[derive(Serialize)]
pub struct News {
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: String,
    pub guid: String,
    pub comments: String,
}
impl News {
    pub fn new() -> News {
        News {
            title: String::new(),
            link: String::new(),
            description: String::new(),
            pub_date: String::new(),
            guid: String::new(),
            comments: String::new()
        }
    }
}
