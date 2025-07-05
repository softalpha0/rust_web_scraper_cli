use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::ChannelId,
        application::{command::Command, interaction::{Interaction, InteractionResponseType}},
    },
    prelude::*,
};
use dotenv::dotenv;
use std::env;
use tokio::time::{interval, Duration};
use crate::scraper::scrape_tokens;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Register global slash commands
        let _ = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|cmd| {
                    cmd.name("ping").description("Check if bot is alive")
                })
                .create_application_command(|cmd| {
                    cmd.name("scan").description("Manually scan for new tokens")
                })
        }).await;

        // Start periodic scanner
        let ctx_clone = ctx.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(300)); // 5 min
            loop {
                interval.tick().await;
                if let Ok(alerts) = try_get_token_alerts().await {
                    for alert in alerts {
                        let _ = ChannelId(1390860816584020070).say(&ctx_clone.http, alert).await;
                    }
                }
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            let name = cmd.data.name.as_str();
            match name {
                "ping" => {
                    let _ = cmd.create_interaction_response(&ctx.http, |r| {
                        r.kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|m| m.content("Pong!"))
                    }).await;
                },
                "scan" => {
                    match try_get_token_alerts().await {
                        Ok(alerts) if !alerts.is_empty() => {
                            for alert in alerts {
                                let _ = cmd.create_interaction_response(&ctx.http, |r| {
                                    r.kind(InteractionResponseType::ChannelMessageWithSource)
                                        .interaction_response_data(|m| m.content(alert))
                                }).await;
                            }
                        },
                        _ => {
                            let _ = cmd.create_interaction_response(&ctx.http, |r| {
                                r.kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|m| m.content("No promising tokens found."))
                            }).await;
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

async fn try_get_token_alerts() -> Result<Vec<String>, ()> {
    let tokens = scrape_tokens().await;
    Ok(tokens.into_iter().map(|t| {
        format!("🟢 **{}** found on **{}**\nHolders: {}\nVolume: {} SOL\nLP: {} SOL", t.name, t.source, t.holders, t.volume, t.lp)
    }).collect())
}