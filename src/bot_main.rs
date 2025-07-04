use std::env;

use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{
        gateway::{Ready, GatewayIntents},
        prelude::*,
        application::interaction::{Interaction, InteractionResponseType},
        application::command::Command,
    },
    prelude::*,
    framework::standard::StandardFramework,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Register slash commands globally
        Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|cmd| {
                cmd.name("ping").description("Replies with Pong!")
            })
        })
        .await
        .expect("Failed to register slash commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            if command.data.name == "ping" {
                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("Pong!"))
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
    println!("Using token: {}", &token[..5]); // Debug only: show first 5 chars

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