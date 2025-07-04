use std::env;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{gateway::Ready, prelude::Interaction},
    prelude::*,
    framework::standard::{macros::group, StandardFramework},
    builder::CreateApplicationCommand,
    model::prelude::command::Command,
    http::Http,
};

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|cmd| {
                    cmd.name("ping")
                        .description("Replies with Pong!")
                })
                .create_application_command(|cmd| {
                    cmd.name("scrape")
                        .description("Run the web scraper on default links")
                })
        })
        .await;

        match commands {
            Ok(cmds) => {
                println!("Global commands registered:");
                for cmd in cmds {
                    println!("- {}", cmd.name);
                }
            }
            Err(why) => eprintln!("Failed to register commands: {:?}", why),
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = handle_command(&ctx.http, &command).await {
                println!("Error handling command: {:?}", why);
            }
        }
    }
}

#[group]
struct General;

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
        .framework(StandardFramework::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn handle_command(http: &Http, command: &ApplicationCommandInteraction) -> serenity::Result<()> {
    match command.data.name.as_str() {
        "ping" => {
            command.create_interaction_response(http, |response| {
                response.interaction_response_data(|msg| msg.content("Pong!"))
            }).await?;
        }

        "scrape" => {
            // TODO: Replace with your real scraping logic
            command.create_interaction_response(http, |response| {
                response.interaction_response_data(|msg| msg.content("Running scraper..."))
            }).await?;

            println!("Scraping initiated by {}", command.user.name);

            // Add your scraper logic or function call here
        }

        _ => {
            command.create_interaction_response(http, |response| {
                response.interaction_response_data(|msg| msg.content("Unknown command"))
            }).await?;
        }
    }

    Ok(())
}