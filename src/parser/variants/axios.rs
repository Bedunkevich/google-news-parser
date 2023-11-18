use std::time::Duration;

use headless_chrome::protocol::cdp::Target::CreateTarget;
use headless_chrome::{Browser, LaunchOptions};

use sanitize_html::rules::predefined::BASIC;
use sanitize_html::sanitize_str;

// use tokio::time::sleep;

use crate::parser;
use crate::parser::models::Post;
use crate::utils;

const WAITING_TIMEOUT_SEC: u64 = 30;

fn log_tabs(browser: &Browser) {
    let tabs = browser.get_tabs().lock().unwrap().clone();

    tabs.into_iter().for_each(|tab| {
        println!("Tab {:?}", tab.get_url());
    });
}

pub async fn parse(host: &str, url_str: &str) -> Result<Post, failure::Error> {
    let _launch_options = LaunchOptions {
        enable_logging: true,
        headless: false,
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
    // let browser = Browser::default().unwrap();

    log_tabs(&browser);

    let params = CreateTarget {
        url: url_str.to_string(),
        width: None,
        height: None,
        browser_context_id: None,
        enable_begin_frame_control: Some(false),
        new_window: Some(false),
        background: Some(true),
    };
    let tab = browser.new_tab_with_options(params).unwrap();

    tab.activate().unwrap();

    // sleep(Duration::from_secs(3)).await;

    log_tabs(&browser);

    // Click "No thanx" button
    match tab.wait_for_element_with_custom_timeout(
        "button.mt-5",
        Duration::from_secs(WAITING_TIMEOUT_SEC),
    ) {
        Ok(element) => {
            element.click().unwrap();
        }
        Err(err) => {
            println!("error: {:?}\n", err);
        }
    }

    let title = tab
        .wait_for_element_with_custom_timeout(
            r#"h1[data-cy="story-headline"]"#,
            Duration::from_secs(WAITING_TIMEOUT_SEC),
        )
        .unwrap()
        .get_inner_text()
        .unwrap();

    utils::blue_log("Title", &title);

    let hero_image = tab
        .wait_for_element_with_custom_timeout(
            r#"img[data-cy="StoryImage"]"#,
            Duration::from_secs(WAITING_TIMEOUT_SEC),
        )
        .unwrap()
        .get_attribute_value("src")
        .unwrap()
        .unwrap();

    utils::blue_log("Hero image", &hero_image);

    let hero_image_caption = tab
        .wait_for_element_with_custom_timeout(
            r#"figcaption"#,
            Duration::from_secs(WAITING_TIMEOUT_SEC),
        )
        .unwrap()
        .get_inner_text()
        .unwrap();

    utils::blue_log("Image caption", &hero_image_caption);

    let article_element = tab
        .wait_for_element_with_custom_timeout(
            r#"figure + div"#,
            Duration::from_secs(WAITING_TIMEOUT_SEC),
        )
        .unwrap();

    let description_html_string = article_element
        .call_js_fn("function() { return this.innerHTML; }", [].to_vec(), true)
        .unwrap()
        .value
        .unwrap()
        .to_string();

    let mut description = sanitize_str(&BASIC, &description_html_string).unwrap();
    description.remove(0);
    description.pop();

    utils::blue_log("description", &description);

    utils::blue_log(">", host);
    utils::blue_log("", url_str);

    Ok(Post {
        title,
        description,
        hero_image: Some(hero_image),
    })
}

#[tokio::test]
async fn test() {
    let permanent_link =
        "https://www.axios.com/2023/11/18/sam-altman-fired-greg-brockman-openai-microsoft";
    let post = parse("axios.com", permanent_link).await.unwrap();

    let enclosure = Option::as_deref(&post.hero_image);

    if false {
        parser::post(&post.title, &post.description, enclosure)
            .await
            .unwrap();
    }

    assert_eq!(
        post.title,
        String::from("What we know on Sam Altman's shock exit from Silicon Valley's hottest firm")
    );
}
