# gnews-rs

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

gnews-rs is a Rust crate that provides a simple interface to retrieve the latest news feed from Google News. It leverages the power of Rust to offer a fast and efficient way to access news articles programmatically.

## Features

- **Easy to Use:** Straightforward API for fetching news headlines and articles.
- **Customizable:** Retrieve by top stories, topic and search.
- **Asynchronous:** Built with Tokio for non-blocking and efficient news retrieval.

## Installation

- Using Cargo:

```
cargo add gnews-rs
```

- Using Cargo.toml
```
[dependencies]
gnews-rs = "0.1.0"
```

## Usage

```

use gnews_rs::NewsClient;

#[tokio::main]
async fn main() {
    let top_stories = NewsClient::get_top_stories().await;

    let topic_world = NewsClient::get_topic(NewsClient::Topic::World).await;
    let topic_health = NewsClient::get_topic(NewsClient::Topic::Health).await;
    let topic_sports = NewsClient::get_topic(NewsClient::Topic::Sports).await;
    let topic_science = NewsClient::get_topic(NewsClient::Topic::Science).await;
    let topic_technology = NewsClient::get_topic(NewsClient::Topic::Technology).await;
    let topic_business = NewsClient::get_topic(NewsClient::Topic::Business).await;
    let topic_entertainment = NewsClient::get_topic(NewsClient::Topic::Entertainment).await;

    let search_news = NewsClient::get_search("rust programming").await;

    for story in search_news{
        println!("Title: {}", story.title);
        println!("Link: {}", story.link);
        println!("Description: {}", story.description);
        println!("Published: {}", story.pub_date);
        println!("guid: {}", story.guid);
        println!("comments: {}", story.comments);
        println!("==============================");
    }
}


```

# Contributing

Thank you for considering contributing to gnews-rs! Contributions are essential for improving the project and making it more valuable for the community.
Feel free to fork/make pull request on the repo, your contributions are most welcome!


## License

The source code for the site is licensed under the MIT license, which you can find in
the ```LICENSE``` file.
