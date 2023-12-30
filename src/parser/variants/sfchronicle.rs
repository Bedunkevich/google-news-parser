use crate::parser::{models::Post, variants::helpers::new_incognito_tab};
use crate::utils;

use super::helpers::{get_browser, get_src_set, log_tabs};

pub async fn parse(host: &str, url_str: &str) -> Result<Option<Post>, failure::Error> {
    let browser = get_browser(false).await.unwrap();

    let tab = new_incognito_tab(&browser, url_str.to_string())
        .await
        .unwrap();

    tab.activate().unwrap();

    log_tabs(&browser);

    let title_element = tab.wait_for_element(r#"section h1"#);

    let title: String;

    match title_element {
        Ok(element) => {
            title = element.get_inner_text().unwrap();
            utils::blue_log("Title", &title);

            let src_set = tab
                .wait_for_element(r#"picture img"#)
                .unwrap()
                .get_attribute_value("srcset")
                .unwrap()
                .unwrap();

            let hero_image = get_src_set(&src_set).unwrap();

            utils::blue_log("Hero image", &hero_image);

            let hero_image_caption = tab
                .wait_for_element(r#"figcaption"#)
                .unwrap()
                .get_inner_text()
                .unwrap();

            utils::blue_log("Image caption", &hero_image_caption);

            let article_elements = tab.wait_for_elements(r#"article p"#).unwrap();

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

            Ok(Some(Post {
                original_link: url_str.to_string(),
                title,
                description,
                hero_image: Some(hero_image.to_string()),
            }))
        }
        Err(e) => {
            println!("Error parse sfchronicle.com: {:?}", e);
            return Ok(None);
        }
    }
}

#[tokio::test]
async fn test() {
    let permanent_link ="https://www.sfchronicle.com/realestate/article/california-city-affordable-home-18577269.php";

    let optional_post = parse("sfchronicle.com", permanent_link).await.unwrap();

    match optional_post {
        Some(post) => {
            assert_eq!(
                post.title,
                String::from("It’s not just the Bay Area — the number of affordable homes is plummeting across California")
            );
        }
        None => {}
    }
}
