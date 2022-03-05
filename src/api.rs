use serde_json::json;
use std::error::Error;

use crate::config::Config;

pub async fn create_todo(config: &Config, title: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let body = json!({
        "parent": json!({
            "database_id": &config.database_id,
        }),
        "properties": json!({
            "title": [
                json!({
                    "text": json!({
                        "content": title
                    })
                })
            ]
        })
    });

    let _ = client
        .post("https://api.notion.com/v1/pages")
        .bearer_auth(&config.secret)
        .header("Notion-Version", "2021-08-16")
        .json(&body)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}
