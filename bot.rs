use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::process::Command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.content == "!scrape" {
            // Example CLI call
            let output = Command::new("./target/debug/web_scraper_cli")
                .args(&[
                    "https://example.com",
                    "--output", "discord_scrape.csv",
                    "--csv",
                ])
                .output()
                .expect("Failed to run web scraper CLI");

            if output.status.success() {
                let _ = msg.channel_id.say(&_ctx.http, "✅ Scrape completed!").await;
            } else {
                let _ = msg.channel_id.say(&_ctx.http, "❌ Failed to run scraper.").await;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in env");

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}