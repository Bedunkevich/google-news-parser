use std::time::Duration;

use headless_chrome::{Browser, Element, LaunchOptions};

use crate::{parser, utils};

const WAITING_TIMEOUT_SEC: u64 = 30;

pub async fn parse(host: &str, url_str: &str) -> Result<(), failure::Error> {
    //    let browser = Browser::default().unwrap();
    let _launch_options = LaunchOptions {
        enable_logging: true,
        headless: true,
        sandbox: true,
        enable_gpu: false,
        window_size: Some((100, 100)),
        port: None,
        ignore_certificate_errors: true,
        path: None,
        user_data_dir: None,
        extensions: [].to_vec(),
        args: [].to_vec(),
        disable_default_args: false,
        idle_browser_timeout: Duration::from_secs(WAITING_TIMEOUT_SEC),
        process_envs: None,
        proxy_server: None,
    };
    let browser = Browser::new(_launch_options).unwrap();

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
        parser::post_to_blockchain(&header_text, &article_text, None)
            .await
            .unwrap();
    }

    let _ = tab.close(true);

    println!();

    return Ok(());
}

#[tokio::test]
async fn test() {
    let permanent_link = "https://nypost.com/2023/11/12/metro/two-men-arrested-after-allegedly-attacking-nypd-cop-on-subway/";
    parse("host", permanent_link).await.unwrap();

    assert_eq!(2 + 3, 5);
}
