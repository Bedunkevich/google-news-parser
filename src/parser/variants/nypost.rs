use headless_chrome::{Browser, Element};

use crate::{parser, utils};

pub async fn parse(host: &str, url_str: &str) -> Result<(), failure::Error> {
    let browser = Browser::default().unwrap();

    let tab = browser.new_tab().unwrap();

    tab.navigate_to(url_str).unwrap();

    tab.wait_until_navigated().unwrap();

    let header_element: Element<'_> = tab.wait_for_element("header.article-header > h1").unwrap();
    let header_text: String = header_element.get_inner_text().unwrap();

    let article_element: Element<'_> = tab
        .wait_for_element("div.single__content.entry-content")
        .unwrap();
    let article_text: String = article_element.get_inner_text().unwrap();

    utils::system_log(">", host);
    utils::blue_log("", url_str);
    utils::black_log("H >", &header_text);
    utils::black_log("T >", &article_text);

    if false {
        parser::post(&header_text, &article_text, None)
            .await
            .unwrap();
    }

    println!();

    return Ok(());
}

#[tokio::test]
async fn test() {
    let permanent_link = "https://nypost.com/2023/11/12/metro/two-men-arrested-after-allegedly-attacking-nypd-cop-on-subway/";
    parse("host", permanent_link).await.unwrap();

    assert_eq!(2 + 2, 4);
}
