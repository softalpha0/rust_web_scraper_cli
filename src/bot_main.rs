use std::env;
use tokio::time::{sleep, Duration};
use serenity::{
    async_trait,
    model::{gateway::Ready, id::ChannelId},
    prelude::*,
};

mod token_scanner;
use token_scanner::{scrape_dexscreener, scrape_pump_fun, test_me};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("✅ Bot is connected.");

        // Get channel ID from env
        let channel_id = ChannelId(
            env::var("DISCORD_CHANNEL_ID")
                .expect("❌ Missing DISCORD_CHANNEL_ID")
                .parse::<u64>()
                .unwrap(),
        );

        // Confirm token_scanner loaded
        test_me();

        // Start scraping task
        tokio::spawn(async move {
            loop {
                // Pump.fun
                if let Ok(tokens) = scrape_pump_fun().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("🟣 Pump.fun token: {}", token)).await;
                    }
                }

                // Dexscreener
                if let Ok(tokens) = scrape_dexscreener().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("🟡 Dexscreener token: {}", token)).await;
                    }
                }

                // Wait 5 minutes
                sleep(Duration::from_secs(300)).await;
            }
        });
    }
}

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}