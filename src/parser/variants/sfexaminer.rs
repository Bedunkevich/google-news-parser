use crate::parser::{models::Post, variants::helpers::new_incognito_tab};
use crate::utils;

use super::helpers::{get_browser, get_src_set, log_tabs};

pub async fn parse(host: &str, url_str: &str) -> Result<Post, failure::Error> {
    let browser = get_browser(false).await.unwrap();

    let tab = new_incognito_tab(&browser, url_str.to_string())
        .await
        .unwrap();

    tab.activate().unwrap();

    log_tabs(&browser);

    let title = tab
        .wait_for_element(r#"header h1"#)
        .unwrap()
        .get_inner_text()
        .unwrap();

    utils::blue_log("Title", &title);

    let src_set = tab
        .wait_for_element(r#"figure img"#)
        .unwrap()
        .get_attribute_value("srcset")
        .unwrap()
        .unwrap();

    let hero_image = get_src_set(&src_set).unwrap();

    utils::blue_log("Hero image", &hero_image);

    let hero_image_caption = tab
        .wait_for_element(r#"figure figcaption"#)
        .unwrap()
        .get_inner_text()
        .unwrap();

    utils::blue_log("Image caption", &hero_image_caption);

    let article_elements = tab.wait_for_elements(r#"div.main-content-wrap p"#).unwrap();

    let description = article_elements
        .iter()
        .enumerate()
        .filter(|&(i, _)| i != 0)
        .map(|(_, element)| {
            let inner_text = element.get_inner_text().unwrap();
            let inner_text_to_lowercase = inner_text.to_lowercase();

            if ["advertisement", "article continues below this ad"]
                .contains(&inner_text_to_lowercase.as_str())
            {
                return String::from("");
            }

            return inner_text;
        })
        .collect::<Vec<String>>()
        .join("<br/>");

    utils::blue_log("description", &description);

    utils::blue_log(">", host);
    utils::blue_log("", url_str);

    Ok(Post {
        original_link: url_str.to_string(),
        title,
        description,
        hero_image: Some(hero_image.to_string()),
    })
}

#[tokio::test]
async fn test() {
    let permanent_link =
        "https://www.sfexaminer.com/news/politics/why-sf-supervisors-may-want-to-sue-california-over-housing/article_1c441194-a672-11ee-9ba9-ab0a9d08e1c1.html";

    let post = parse("sfexaminer.com", permanent_link).await.unwrap();

    assert_eq!(
        post.title,
        String::from("Why two SF supervisors may want to sue California")
    );
}
