use colored::Colorize;
use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest;
use serde_json;
use std::collections::HashMap;

use crate::{google_news::Article, utils};

mod models;
mod variants;

pub async fn parse_article(article: Article) -> Result<(), ExitFailure> {
    let host = article.host.as_str();
    let original_link = article.original_link.as_str();

    match host {
        "www.foxnews.com" => {
            println!();
            variants::fox_news::parse(host, original_link).await?;
        }
        "www.curbed.com" => {
            variants::curbed::parse(host, original_link).await?;
        }
        "nypost.com" => {
            variants::nypost::parse(host, original_link).await?;
        }
        "www.axios.com" => {
            variants::axios::parse(host, original_link).await?;
        }
        _ => {
            utils::black_log(">", original_link);
        }
    }

    return Ok(());
}

pub async fn post(
    title: &str,
    description: &str,
    enclosure: Option<&str>,
) -> Result<(), failure::Error> {
    dotenv().ok();
    let token = std::env::var("API_TOKEN").unwrap_or(String::from("token"));
    let token_clone = token.clone();
    let mut map = HashMap::new();
    map.insert("title", title);
    map.insert("description", description);
    map.insert("enclosure", enclosure.unwrap());

    map.insert("publicationType", "POST");

    let client = reqwest::Client::new();
    let result = client
        .post("https://summary.news/api/posts/new")
        .bearer_auth(token)
        .json(&map)
        .send()
        .await;

    match result {
        Ok(data) => {
            let json = data.json::<serde_json::Value>().await.unwrap();

            println!(
                "{}\n{}",
                token_clone.on_bright_black(),
                serde_json::to_string_pretty(&json).unwrap()
            );
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }

    return Ok(());
}

#[tokio::test]
async fn post_test() {
    // post("A", "B").await.unwrap();

    assert_eq!(2 + 2, 4);
}
