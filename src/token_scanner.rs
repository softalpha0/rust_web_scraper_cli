use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

pub async fn scrape_pump_fun() -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://pump.fun/";
    let html = Client::new().get(url).send().await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a[href]").unwrap();

    let mut tokens = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.contains("/token/") {
                tokens.push(href.to_string());
            }
        }
    }

    Ok(tokens)
}

pub async fn scrape_dexscreener() -> Result<Vec<String>, Box<dyn Error>> {
    let url = "https://api.dexscreener.com/latest/dex/pairs/solana";
    let json = Client::new().get(url).send().await?.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&json)?;

    let mut tokens = Vec::new();
    if let Some(pairs) = parsed.get("pairs").and_then(|p| p.as_array()) {
        for pair in pairs.iter().take(10) {
            if let Some(name) = pair.get("baseToken").and_then(|b| b.get("name")) {
                tokens.push(name.to_string());
            }
        }
    }

    Ok(tokens)
}