use exitfailure::ExitFailure;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

use crate::utils;

pub async fn parse(host: &str, url_str: &str) -> Result<(), ExitFailure> {
    let fetch_url: Url = Url::parse(url_str)?;

    let html: String = reqwest::get(fetch_url).await?.text().await?;
    let document = Html::parse_document(&html);
    let article: ElementRef<'_> = document
        .select(&Selector::parse(r#".article-body"#).unwrap())
        .next()
        .unwrap();

    // print!("{:?}", article.html());

    println!();
    utils::blue_log(">", host);
    utils::blue_log("", url_str);

    let mut index: i32 = 1;
    let selector = Selector::parse("p").unwrap();

    for element in article.select(&selector) {
        let text = element
            .children()
            .filter_map(|child| ElementRef::wrap(child))
            .flat_map(|el| el.text())
            //.collect::<Vec<_>>();
            .collect::<String>();

        utils::log(format!("{index}").as_str(), &text);
        index += 1;
    }

    return Ok(());
}
