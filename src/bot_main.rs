use std::env;
use tokio::time::{sleep, Duration};
use serenity::{
    async_trait,
    model::{gateway::Ready, id::ChannelId},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("Bot is connected.");

        let channel_id = ChannelId(
            env::var("DISCORD_CHANNEL_ID")
                .expect("Missing DISCORD_CHANNEL_ID")
                .parse::<u64>()
                .unwrap(),
        );

        tokio::spawn(async move {
            loop {
                // Scrape Pump.fun
                if let Ok(tokens) = token_scanner::scrape_pump_fun().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("Pump.fun token: {}", token)).await;
                    }
                }

                // Scrape Dexscreener
                if let Ok(tokens) = token_scanner::scrape_dexscreener().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("Dexscreener token: {}", token)).await;
                    }
                }

                // Wait for 5 minutes
                sleep(Duration::from_secs(300)).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(err) = client.start().await {
        eprintln!("Client error: {:?}", err);
    }
}