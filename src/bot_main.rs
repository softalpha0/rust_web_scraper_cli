use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use dotenv::dotenv;
use std::env;
use tokio::time::{sleep, Duration};

mod token_scanner;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        println!("Bot is connected.");

        let channel_id = ChannelId(env::var("DISCORD_CHANNEL_ID")
            .expect("Missing DISCORD_CHANNEL_ID")
            .parse::<u64>()
            .unwrap());

        tokio::spawn(async move {
            loop {
                if let Ok(tokens) = token_scanner::scrape_pump_fun().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("Pump.fun token: {token}")).await;
                    }
                }
                if let Ok(tokens) = token_scanner::scrape_dexscreener().await {
                    for token in tokens.iter().take(5) {
                        let _ = channel_id.say(&ctx.http, format!("Dexscreener token: {token}")).await;
                    }
                }

                sleep(Duration::from_secs(300)).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}