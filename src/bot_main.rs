 use serenity::{
  async_trait,
  model::{
      gateway::Ready,
      prelude::{Command, Interaction, InteractionResponseType},
  },
  prelude::*,
  builder::CreateApplicationCommand,
  http::Http,
};

use std::env;
use dotenv::dotenv;

mod scraper; // Make sure scrape.rs is in the same folder and `pub mod scraper;` is in lib.rs if needed

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
      println!("{} is connected!", ready.user.name);

      let http = &ctx.http;

      let global_commands = Command::set_global_application_commands(http, |commands| {
          commands
              .create_application_command(|command| {
                  command
                      .name("ping")
                      .description("Test if the bot is responsive.")
              })
              .create_application_command(|command| {
                  command
                      .name("scrape")
                      .description("Run the web scraper and return result.")
              })
      })
      .await;

      match global_commands {
          Ok(_) => println!("Global slash commands set."),
          Err(e) => println!("Error setting commands: {:?}", e),
      }
  }

  async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
      if let Interaction::ApplicationCommand(command) = interaction {
          let content = match command.data.name.as_str() {
              "ping" => "Pong!".to_string(),
              "scrape" => {
                  let client = reqwest::Client::new();
                  let url = "https://news.ycombinator.com/"; // Or from env/config
                  let scraped = scraper::scrape_url(&client, url.to_string()).await;
                  format!("Scraped {} items!", scraped.len())
              }
              _ => "Unknown command".to_string(),
          };

          if let Err(err) = command
              .create_interaction_response(&ctx.http, |response| {
                  response.kind(InteractionResponseType::ChannelMessageWithSource)
                      .interaction_response_data(|message| message.content(content))
              })
              .await
          {
              println!("Failed to respond to interaction: {}", err);
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

  let framework = serenity::framework::standard::StandardFramework::new();

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