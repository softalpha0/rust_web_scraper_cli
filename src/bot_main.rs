use dotenv::dotenv;
use std::env;
use std::time::Duration;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tokio::time::sleep;

mod token_scanner;
use token_scanner::run_scanner;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("✅ Bot is connected.");

        // Start the 5-minute loop
        tokio::spawn(async {
            loop {
                run_scanner().await;
                sleep(Duration::from_secs(300)).await;
            }
        });

        println!("✅ token_scanner module loaded successfully!");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = match env::var("DISCORD_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("❌ DISCORD_TOKEN not found in .env");
            return;
        }
    };

    let mut client = Client::builder(&token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .await
        .expect("❌ Failed to create Discord client");

    if let Err(why) = client.start().await {
        eprintln!("❌ Client error: {:?}", why);
    }
}