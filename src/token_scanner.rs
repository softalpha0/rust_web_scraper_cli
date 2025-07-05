use reqwest::Client;
use scraper::{Html, Selector};

pub async fn scrape_pump_fun() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let body = client
        .get("https://pump.fun/")
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href^='/token/']").unwrap();

    let tokens: Vec<String> = document
        .select(&selector)
        .filter_map(|elem| elem.value().attr("href"))
        .map(|href| format!("https://pump.fun{}", href))
        .collect();

    Ok(tokens)
}

pub async fn scrape_dexscreener() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://api.dexscreener.com/latest/dex/pairs/solana")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let tokens: Vec<String> = response["pairs"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .take(5)
        .filter_map(|pair| pair["url"].as_str().map(|s| s.to_string()))
        .collect();

    Ok(tokens)
}