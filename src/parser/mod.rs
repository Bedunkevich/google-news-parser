use colored::Colorize;
use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest;
use serde_json;
use std::collections::HashMap;

use crate::{google_news::Article, utils};

use self::models::Post;

mod models;
mod variants;

const USE_POST: bool = true;

pub async fn parse_article(article: Article) -> Result<(), ExitFailure> {
    let host_string = get_host(article.host).unwrap();
    let host = host_string.as_str();
    let original_link = article.original_link.as_str();

    match host {
        "foxnews.com" => {
            println!();
            variants::fox_news::parse(host, original_link).await?;
        }
        "curbed.com" => {
            variants::curbed::parse(host, original_link).await?;
        }
        "nypost.com" => {
            variants::nypost::parse(host, original_link).await?;
        }
        "axios.com" => {
            variants::axios::parse(host, original_link).await?;
        }
        "sfchronicle.com" => {
            let is_article_link = original_link.contains("article");

            if is_article_link {
                match variants::sfchronicle::parse(host, original_link).await {
                    Ok(optional_post) => {
                        if USE_POST {
                            match optional_post {
                                Some(post) => {
                                    commit_post(&post).await?;
                                }
                                None => {}
                            }
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            } else {
                utils::black_log("Non article", original_link);
            }
        }
        "sfstandard.com" => match variants::sfstandard::parse(host, original_link).await {
            Ok(post) => {
                if USE_POST {
                    commit_post(&post).await?;
                }
            }
            Err(e) => println!("Error: {:?}", e),
        },
        "sfexaminer.com" => {
            let not_article_link = original_link.contains("local-events");

            if not_article_link {
                utils::black_log("Non article", original_link);
            } else {
                match variants::sfexaminer::parse(host, original_link).await {
                    Ok(post) => {
                        if USE_POST {
                            commit_post(&post).await?;
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
        _ => {
            utils::black_log("?", host);
        }
    }

    return Ok(());
}

pub async fn commit_post(post: &Post) -> Result<(), failure::Error> {
    post_to_blockchain(&post.title, &post.description, post.hero_image.as_deref()).await?;
    Ok(())
}

pub async fn post_to_blockchain(
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

pub fn get_host(origin: String) -> Result<String, failure::Error> {
    // let origin = String::from("www_foxnews_com");
    let parts = origin.split(".");
    let collection: Vec<&str> = parts.collect();
    let len = collection.len();

    let last = &collection[len - 2..len];
    let host: String = last.join(".");

    return Ok(host);
}

#[tokio::test]
async fn get_host_test() {
    let host = get_host("www.foxnews.com".to_string()).unwrap();

    assert_eq!(host, "foxnews.com");
}
