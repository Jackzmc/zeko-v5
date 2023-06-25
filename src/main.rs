use std::any::Any;
use dotenv::dotenv;
use serde_json::json;

use serenity::{async_trait, model};
use serenity::model::channel::Channel;
use serenity::Client;
use serenity::model::prelude::*;
use serenity::prelude::*;

mod util;
mod commands;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot { return; }

    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(std::env::var("GUILD_ID").expect("Expected GUILD_ID in environment").parse().expect("GUILD_ID must be an integer"));
        println!("[Discord] Connected as {}", ready.user.name);
        let commands = guild_id.set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
        }).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: interaction::Interaction) {
        if let Some(cmd) = interaction.application_command() {
            cmd.create_interaction_response(&ctx.http, |r|
                r.kind(model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("PONG"))
            ).await.ok();
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("no discord token provided");
    let application_id = std::env::var("DISCORD_ID").expect("no discord id provided");


    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;



    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .application_id(application_id.parse::<u64>().expect("invalid client id"))
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        // data.insert::<ChatServerContainer>(manager);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
