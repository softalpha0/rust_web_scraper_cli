use serenity::{
    async_trait,
    model::{gateway::Ready, application::interaction::{Interaction, InteractionResponseType}, application::command::Command},
    prelude::*,
    builder::CreateApplicationCommand,
    http::Http,
};

use std::env;
use dotenv::dotenv;

mod scraper;
use scraper::{scrape_url, OutputFormat};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("Test if the bot is responsive.")
                })
                .create_application_command(|command| {
                    command.name("scrape")
                        .description("Run the scraper and return result.")
                })
        })
        .await;

        if let Err(why) = global_commands {
            println!("Error setting global slash commands: {:?}", why);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "ping" => {
                    if let Err(why) = command.create_interaction_response(&ctx.http, |resp| {
                        resp.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg| msg.content("Pong!"))
                    }).await {
                        println!("Failed to respond to ping: {:?}", why);
                    }
                }

                "scrape" => {
                    // Simple URL and client
                    let url = "https://news.ycombinator.com";
                    let client = reqwest::Client::new();
                    let items = scrape_url(&client, url).await;

                    let response = format!(
                        "Scraped {} items from {}",
                        items.len(),
                        url
                    );

                    if let Err(why) = command.create_interaction_response(&ctx.http, |resp| {
                        resp.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg| msg.content(response))
                    }).await {
                        println!("Failed to respond to scrape: {:?}", why);
                    }
                }

                _ => {}
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected APPLICATION_ID in .env")
        .parse()
        .expect("APPLICATION_ID must be a u64");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}