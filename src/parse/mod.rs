use exitfailure::ExitFailure;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

pub async fn get_article_link(id: &str) -> Result<String, ExitFailure> {
    let url: String = format!("https://news.google.com/rss/articles/{id}?oc=1");

    let fetch_url: Url = Url::parse(&*url)?;

    let html: String = reqwest::get(fetch_url).await?.text().await?;
    let document = Html::parse_document(&html);

    let select_c_wiz: Selector = Selector::parse(r#"c-wiz"#).unwrap();
    let c_wiz: ElementRef<'_> = document.select(&select_c_wiz).next().unwrap();
    let link: &str = c_wiz.value().attr("data-n-au").unwrap();

    return Ok(link.to_string());
}
