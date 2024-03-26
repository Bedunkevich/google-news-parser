#[macro_use]
extern crate rocket;
use colored::Colorize;
use dotenv::dotenv;
use exitfailure::ExitFailure;
use feed_rs;
use feed_rs::model::Entry;
use feed_rs::model::Feed;
use opendal::layers::LoggingLayer;
use opendal::services::Mysql;
use opendal::Operator;
use reqwest::Url;
use rusqlite::{Connection, Result};
use std::env;

mod google_news;
mod messages;
mod parser;
mod utils;

const MAX_ITEMS: usize = 10;

#[tokio::main]
async fn start() -> Result<(), ExitFailure> {
    let mut tasks = vec![];
    let args: Vec<String> = env::args().collect();
    let url: &str;
    let data_base = "./data.db3";

    if args.len() < 2 {
        messages::help();
        return Ok(());
    }

    if !args[1].contains("--url") {
        println!("{} {}", "Unknow options:".yellow(), args[1]);
        return Ok(());
    }

    if !args.get(2).is_some() {
        println!("{} - {:?}", "url params not found".yellow(), args);
        return Ok(());
    }

    // All arguments are present
    dotenv().ok();

    // Open the database connection
    let conn = Connection::open(data_base)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;

    // OpenDal mySQL

    // let mut builder = Mysql::default();
    // builder.connection_string("mysql://root:smartmedia@127.0.0.1:3306/fetcher");
    // builder.table("sources");
    // builder.key_field("key");
    // builder.value_field("value");

    // let op = Operator::new(builder)?
    //     .layer(LoggingLayer::default())
    //     .finish();

    // op.write("hello.txt", "Hello, World!").await?;

    url = &args[2];
    utils::system_log("Fetching", url);

    let fetch_url: Url = Url::parse(&*url)?;

    let text: String = reqwest::get(fetch_url).await?.text().await?;
    let feed: Feed = feed_rs::parser::parse(text.as_bytes()).unwrap();
    let entries: Vec<Entry> = feed.entries;

    utils::log("title", &feed.title.unwrap().content);

    let mut i: usize = 0;

    for item in entries {
        tasks.push(tokio::spawn(async move {
            let link = String::from(&item.links[0].href);
            let result = google_news::get_article_link(&link, None).await;

            match result {
                Ok(link) => {
                    let id = &item.id;

                    utils::log("id", id);
                    utils::log("title", &item.title.unwrap().content);
                    utils::log("published", &item.published.unwrap().to_string());
                    utils::log("link", &link.original_link);
                    utils::log("host", &link.host);
                    println!("");
                    return link;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    panic!()
                }
            }
        }));

        i += 1;

        if i >= MAX_ITEMS {
            break;
        }
    }

    println!("");

    let mut articles = vec![];

    for task in tasks {
        articles.push(task.await.unwrap());
    }

    utils::system_log(">", "Parsing sources....");

    let mut sites = vec![];

    for article in articles {
        sites.push(tokio::spawn(async move {
            let result = parser::parse_article(article).await;

            match result {
                Ok(()) => {
                    // ToDo
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }));
    }

    for site in sites {
        match site.await {
            Ok(()) => {
                // ToDo
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    // println!("results: {:?}", results);
    Ok(())
}

#[get("/")]
fn hello() {
    main()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
