use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct TokenAlert {
    pub source: String,
    pub name: String,
    pub url: String,
    pub volume: Option<f64>,
    pub holders: Option<u32>,
    pub liquidity: Option<f64>,
}

pub async fn try_get_token_alerts() -> Result<Vec<TokenAlert>, Box<dyn Error>> {
    let mut alerts = Vec::new();

    // --- Pump.fun scraper ---
    if let Ok(pump_resp) = Client::new()
        .get("https://pump.fun/api/token/list")
        .send()
        .await
    {
        if let Ok(pump_json) = pump_resp.json::<serde_json::Value>().await {
            if let Some(tokens) = pump_json.as_array() {
                for token in tokens.iter().take(5) {
                    let name = token.get("name").and_then(|v| v.as_str()).unwrap_or("Unnamed");
                    let id = token.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    let holders = token.get("holders").and_then(|v| v.as_u64()).unwrap_or(0);
                    let volume = token.get("volume").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let url = format!("https://pump.fun/{}", id);

                    // Simple filter logic (you can tweak thresholds)
                    if holders > 20 && volume > 10.0 {
                        alerts.push(TokenAlert {
                            source: "Pump.fun".into(),
                            name: name.into(),
                            url,
                            holders: Some(holders as u32),
                            volume: Some(volume),
                            liquidity: None,
                        });
                    }
                }
            }
        }
    }

    // --- DexScreener scraper ---
    if let Ok(resp) = Client::new()
        .get("https://api.dexscreener.com/latest/dex/pairs/solana")
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(pairs) = json["pairs"].as_array() {
                for pair in pairs.iter().take(5) {
                    let name = pair["baseToken"]["name"].as_str().unwrap_or("Unnamed");
                    let url = pair["url"].as_str().unwrap_or("");
                    let volume_usd = pair["volume"]["h1"]
                        .as_f64()
                        .or_else(|| pair["volume"]["h6"].as_f64())
                        .unwrap_or(0.0);
                    let liquidity = pair["liquidity"]["usd"].as_f64().unwrap_or(0.0);

                    if volume_usd > 1000.0 && liquidity > 1000.0 {
                        alerts.push(TokenAlert {
                            source: "DexScreener".into(),
                            name: name.into(),
                            url: url.into(),
                            volume: Some(volume_usd),
                            liquidity: Some(liquidity),
                            holders: None,
                        });
                    }
                }
            }
        }
    }

    Ok(alerts)
}