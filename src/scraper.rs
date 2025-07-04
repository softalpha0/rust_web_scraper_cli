use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ScrapedItem {
    pub url: String,
    pub value: String,
    pub target: String,
}

#[derive(Debug)]
pub enum OutputFormat {
    Csv,
    Json,
}

pub async fn scrape_url(client: &Client, url: String) -> Vec<ScrapedItem> {
    let mut items = Vec::new();
    match client.get(&url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let document = Html::parse_document(&text);
                let selector = Selector::parse("a").unwrap();
                for element in document.select(&selector) {
                    if let Some(link) = element.value().attr("href") {
                        items.push(ScrapedItem {
                            url: url.clone(),
                            value: link.to_string(),
                            target: "link".to_string(),
                        });
                    }
                }
            }
            Err(_) => eprintln!("❌ Failed to parse HTML from {}", url),
        },
        Err(_) => eprintln!("❌ Request failed for {}", url),
    }

    items
}
