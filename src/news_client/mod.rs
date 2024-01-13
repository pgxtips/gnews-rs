mod defs;
mod constants;

use constants::{TOP_STORIES_RSS, TOPICS_RSS, SEARCH_RSS};

use constants::Topic as Topic;
use defs::News as News;


use std::error::Error;
use rss::Channel;

async fn get_top_stories_rss() -> Result<Channel, Box<dyn Error>>{
    let resp = reqwest::get(TOP_STORIES_RSS).await?.bytes().await?;
    let channel = Channel::read_from(&resp[..])?;
    Ok(channel)
}

async fn get_topic_rss(topic: Topic) -> Result<Channel, Box<dyn Error>>{
    let mut request_url = TOPICS_RSS.to_owned();
    request_url.push_str(&topic.val());
    let client = reqwest::Client::new();
    let resp = client.get(request_url)
        .header("User-Agent", "gnews-rs")
        .send()
        .await?.bytes().await?;
    let channel = Channel::read_from(&resp[..])?;
    Ok(channel)
}

async fn get_search_rss(query: &str) -> Result<Channel, Box<dyn Error>>{
    let mut request_url = SEARCH_RSS.to_owned();
    request_url.push_str(query);
    let client = reqwest::Client::new();
    let resp = client.get(request_url)
        .header("User-Agent", "gnews-rs")
        .send()
        .await?.bytes().await?;
    let channel = Channel::read_from(&resp[..])?;
    Ok(channel)
}

fn process_news_channel(channel: Result<Channel, Box<dyn Error>>) -> Result<Vec<News>, Box<dyn Error>>{

    let feed = channel?;
    let mut news: Vec<News> = Vec::new();

    for item in feed.items(){
        let mut new_item: News = News::new();
        new_item.title = item.title().unwrap_or_default().to_string();
        new_item.link = item.link().unwrap_or_default().to_string();
        new_item.description = item.description().unwrap_or_default().to_string();
        new_item.pub_date = item.pub_date().unwrap_or_default().to_string();
        new_item.guid = item.guid().unwrap_or(&rss::Guid::default()).value().to_string();
        new_item.comments = item.comments().unwrap_or_default().to_string();
        news.push(new_item);
    }

    Ok(news)
}


pub mod NewsClient {
    use super::*;
    pub use crate::news_client::constants::Topic as Topic;
    pub use crate::news_client::defs::News as News;

    pub async fn get_top_stories() -> Vec<News>{
        let rss = get_top_stories_rss().await;
        let news = process_news_channel(rss);

        match news {
            Ok(news) => news,
            Err(e) => {
                log::error!("Error: {}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_topic(topic: Topic) -> Vec<News>{
        let rss = get_topic_rss(topic).await;
        let news = process_news_channel(rss);

        match news {
            Ok(news) => news,
            Err(e) => {
                log::error!("Error: {}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_search(query: &str) -> Vec<News>{
        let rss = get_search_rss(query).await;
        let news = process_news_channel(rss);

        match news {
            Ok(news) => news,
            Err(e) => {
                log::error!("Error: {}", e);
                Vec::new()
            }
        }
    }
} 
