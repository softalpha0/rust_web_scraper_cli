mod bot_main;
mod scraper;

use bot_main::Handler;
use serenity::prelude::*;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let app_id: u64 = env::var("APPLICATION_ID").expect("Missing APPLICATION_ID").parse().unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .application_id(app_id)
        .event_handler(Handler)
        .await
        .expect("Error building client");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}