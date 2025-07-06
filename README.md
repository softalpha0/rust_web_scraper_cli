# 🕸️ web_scraper_cli

A powerful CLI tool to scrape webpages and export links, metadata, and more — built in Rust. Outputs results in CSV or JSON. Great for automation, research, and integrations.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat&logo=rust)
![Platform](https://img.shields.io/badge/platform-CLI-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

---

## 🚀 Features

- 🔗 Scrapes all links from provided URLs
- 🏷️ Extracts link text (value) and target
- 🧾 Outputs to **CSV** or **JSON**
- 🧠 Multi-URL scraping via command-line
- 📦 Auto-generated filenames with timestamps
- ⚙️ Built in Rust for speed and reliability

---

## 🧑‍💻 Usage

```bash
cargo run -- https://example.com https://another-site.com \
  --output links.csv --csv
  # 🕵️‍♂️ web_scraper_cli

  ## 🧠 Token Scanning Discord Bot (Rust)

  A Rust-powered Discord bot that scrapes token listings from:

  - [Pump.fun](https://pump.fun)
  - [Dexscreener](https://dexscreener.com)

  And sends live alerts every 5 minutes to a Discord channel.

  ---

  ## ✨ Features

  - Scrapes newly launched tokens
  - Sends alerts to Discord with basic info
  - Modular scraping logic via `token_scanner.rs`
  - Built with `serenity`, `tokio`, and `scraper`
  - Easily deployable with Docker

  ---

  ## 🛠️ Setup

  ### 1. Clone and setup

  ```bash
  git clone https://github.com/softalpha0/rust_web_scraper_cli.git
  cd rust_web_scraper_cli