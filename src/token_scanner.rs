    // src/token_scanner.rs

    use reqwest::Client;
    use scraper::{Html, Selector};
    use std::error::Error;

    /// ✅ Test function to confirm this module is loaded properly
    pub fn test_me() {
        println!("✅ token_scanner module loaded successfully!");
    }

    /// Asynchronously scrapes tokens from Pump.fun
    pub async fn scrape_pump_fun() -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let url = "https://pump.fun";
        let body = Client::new().get(url).send().await?.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a[href*=\"/token/\"]").unwrap();

        let tokens: Vec<String> = document
            .select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .filter_map(|href| href.strip_prefix("/token/"))
            .map(|token| token.to_string())
            .collect();

        Ok(tokens)
    }

    /// Asynchronously scrapes tokens from DexScreener (example: trending pairs)
    pub async fn scrape_dexscreener() -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        let url = "https://api.dexscreener.com/latest/dex/pairs/solana";
        let resp = Client::new().get(url).send().await?.json::<serde_json::Value>().await?;

        let mut tokens = Vec::new();

        if let Some(pairs) = resp.get("pairs").and_then(|v| v.as_array()) {
            for pair in pairs.iter().take(5) {
                if let Some(name) = pair.get("pairCreatedAt").or_else(|| pair.get("baseToken")?.get("name")) {
                    tokens.push(name.to_string());
                }
            }
        }

        Ok(tokens)
    }