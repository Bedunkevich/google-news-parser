use colored::Colorize;
use serde_json::json;

pub fn help() {
    let json: serde_json::Value = json!({
        "common": {
            "--html": [
                "url",
            ],
        }
    });

    println!("{}", "Help".yellow());

    println!(
        "{}{}",
        "Json".bright_green(),
        serde_json::to_string_pretty(&json).unwrap()
    );
}
