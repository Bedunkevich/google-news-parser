use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub original_link: String,
    pub title: String,
    pub description: String,
    pub hero_image: Option<String>,
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0: <50}\t{1: <50}\t{2: <40}\n{3}\n",
            self.original_link.blue(),
            self.title.blue(),
            self.hero_image
                .clone()
                .unwrap_or(String::from("none"))
                .blue(),
            self.description.black()
        )
    }
}

#[tokio::test]
async fn test() {
    let post = Post {
        original_link: String::from("https://original.link/com"),
        title: String::from("Post title"),
        description: String::from("description..."),
        hero_image: Some(String::from("hero.jpg")),
    };

    println!("{}", post)
}
