use exitfailure::ExitFailure;

use crate::{google_news::Article, utils};

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
            variants::curbed::parse(
                host,
                "https://www.curbed.com/article/best-legal-weed-shops-nyc.html",
            )
            .await?;
        }
        _ => {
            utils::black_log(">", original_link);
        }
    }

    return Ok(());
}
