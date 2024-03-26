use exitfailure::ExitFailure;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};
use tokio::time::{sleep, Duration};

pub struct Article {
    pub original_link: String,
    pub host: String,
}

///  Grabs the original article link from the google news
///
/// `id` - Google news articles ID.
///
/// `delay` – optional delay in milliseconds.
pub async fn get_article_link(url: &str, delay: Option<u64>) -> Result<Article, ExitFailure> {
    // let url: String = format!("https://news.google.com/rss/articles/{id}?oc=1");

    let fetch_url: Url = Url::parse(&*url)?;

    if let Some(sleep_millis) = delay {
        sleep(Duration::from_millis(sleep_millis)).await;
    }

    let html: String = reqwest::get(fetch_url).await?.text().await?;
    let document = Html::parse_document(&html);

    let select_c_wiz: Selector = Selector::parse(r#"c-wiz"#).unwrap();
    let c_wiz: ElementRef<'_> = document.select(&select_c_wiz).next().unwrap();
    let link: &str = c_wiz.value().attr("data-n-au").unwrap();

    let link_url: Url = Url::parse(&link)?;

    let host = link_url.host().unwrap();

    return Ok(Article {
        original_link: String::from(link),
        host: String::from(host.to_string()),
    });
}

#[tokio::test]
async fn test() {
    use kafka::producer::Record;
    use kafka::{client::RequiredAcks, producer::Producer};
    use std::fmt::Write;

    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut buf = String::with_capacity(2);
    for i in 0..10 {
        let _ = write!(&mut buf, "{}", i); // some computation of the message data to be sent
        producer
            .send(&Record::from_value("test.get.rss", buf.as_bytes()))
            .unwrap();
        buf.clear();
    }
}
