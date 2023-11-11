use headless_chrome::{Browser, Element};

use crate::utils;

pub async fn parse(host: &str, url_str: &str) -> Result<(), failure::Error> {
    let browser = Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    let message = format!("navigate_to {url_str}");

    utils::system_log("Browser", &message);

    tab.navigate_to(url_str).unwrap();

    tab.wait_until_navigated().unwrap();

    let article: Element<'_> = tab.wait_for_element("div.article-content.inline").unwrap();

    let text: String = article.get_inner_text().unwrap();

    // let text = article
    //     .call_js_fn("function() { return this.innerText;}", [].to_vec(), true)
    //     .unwrap()
    //     .value
    //     .unwrap();

    println!("article: {:?}", article);
    println!("{}", &text);
    utils::blue_log(">", host);
    utils::blue_log("", url_str);

    return Ok(());
}

#[tokio::test]
async fn test() {
    let permanent_link = "https://www.curbed.com/article/best-legal-weed-shops-nyc.html";
    parse("host", permanent_link).await.unwrap();

    assert_eq!(2 + 2, 4);
}
