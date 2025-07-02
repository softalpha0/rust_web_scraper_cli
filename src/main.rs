use std::{env, fs::File, io::Write, path::PathBuf}; // if not used
use chrono::Utc; // if not used
use futures::future::join_all; // if unused
use clap::{Arg, ArgAction, Command};
use reqwest::Client;

mod scrape;
use scrape::{scrape_url, OutputFormat, ScrapedItem};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("web_scraper_cli")
        .version("1.0")
        .about("Scrapes links from webpages")
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .help("Output filename (default: scrape-<timestamp>.csv or .json)")
                .num_args(1),
        )
        .arg(
            Arg::new("csv")
                .long("csv")
                .help("Export to CSV instead of JSON")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("urls")
                .help("Target URLs to scrape")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    let urls: Vec<_> = matches
        .get_many::<String>("urls")
        .unwrap()
        .map(String::from)
        .collect();

    let export_format = if matches.get_flag("csv") {
        OutputFormat::Csv
    } else {
        OutputFormat::Json
    };

    let output = matches
        .get_one::<String>("output")
        .cloned()
        .unwrap_or_else(|| {
            let timestamp = Utc::now().format("%Y%m%d-%H%M%S").to_string();
            match export_format {
                OutputFormat::Csv => format!("scrape-{}.csv", timestamp),
                OutputFormat::Json => format!("scrape-{}.json", timestamp),
            }
        });

    let client = Client::new();
    let tasks = urls.iter().map(|url| scrape_url(&client, url.clone()));
    let results: Vec<ScrapedItem> = futures::future::join_all(tasks)
        .await
        .into_iter()
        .flatten()
        .collect();

    match export_format {
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_path(&output)?;
            wtr.write_record(&["url", "type", "value", "target"])?;
            for item in results {
                wtr.write_record(&[item.url, "link".to_string(), item.value, item.target])?;
            }
            wtr.flush()?;
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&results)?;
            let mut file = File::create(&output)?;
            file.write_all(json.as_bytes())?;
        }
    }

    println!("✅ Scrape complete! Saved to: {}", output);
    Ok(())
}