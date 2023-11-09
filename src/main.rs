use colored::Colorize;
use exitfailure::ExitFailure;
use feed_rs::model::Entry;
use feed_rs::model::Feed;
use feed_rs::parser;
use reqwest::Url;
use std::env;

mod messages;
mod parse;
mod utils;

const MAX_ITEMS: i32 = 10;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args: Vec<String> = env::args().collect();
    let url: &str;

    if args.len() < 2 {
        messages::help();
        return Ok(());
    }

    if !args[1].contains("--url") {
        println!("{} {}", "Unknow options:".yellow(), args[1]);
        return Ok(());
    }

    if !args.get(2).is_some() {
        println!("{}", "url params not found".yellow());
        return Ok(());
    }

    url = &args[2];

    utils::system_log("Fetching", url);

    let fetch_url: Url = Url::parse(&*url)?;

    let text: String = reqwest::get(fetch_url).await?.text().await?;
    let feed: Feed = parser::parse(text.as_bytes()).unwrap();
    let entries: Vec<Entry> = feed.entries;

    utils::log("title", &feed.title.unwrap().content);

    let mut i: i32 = 0;

    for item in entries {
        let content = &item.content;
        let id = &item.id;

        utils::log("id", id);
        utils::log("title", &item.title.unwrap().content);
        // utils::log("summary", &item.summary.unwrap().content);
        utils::log("published", &item.published.unwrap().to_string());

        if content.is_some() {
            println!("Content: {:?}", content);
        }

        let link: String = parse::get_article_link(id).await.unwrap();
        utils::log("link", &link);

        println!("");
        i += 1;

        if i >= MAX_ITEMS {
            break;
        }
    }

    Ok(())
}
