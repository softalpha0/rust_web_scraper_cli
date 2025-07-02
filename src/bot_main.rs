use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::framework::standard::StandardFramework;
use std::env;

// --- Add this bot handler ---
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
// -----------------------------

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load from .env file

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected APPLICATION_ID in .env")
        .parse()
        .expect("APPLICATION_ID must be a u64");

    let framework = StandardFramework::new();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .application_id(application_id)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}