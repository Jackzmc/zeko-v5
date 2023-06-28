use std::any::Any;
use std::sync::{Arc, OnceLock};
use dotenv::dotenv;
use serde_json::json;

use serenity::{async_trait, model};
use serenity::model::channel::Channel;
use serenity::Client;
use serenity::http::CacheHttp;
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::commands::ping::{PingCommand, SlashCommand};

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
        for cmd in COMMANDS.get().unwrap().iter() {
            guild_id.set_application_commands(ctx.http(), |commands| {
                commands.create_application_command(|command| cmd.register(command))
            }).await.unwrap();
        }
        // let commands = guild_id.set_application_commands(&ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::ping::register(command))
        // }).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: interaction::Interaction) {
        if let Some(interact_cmd) = interaction.application_command() {
            let app_data = ctx.data.read().await;;
            // let data = app_data.get::<BotDataContainer>().unwrap();
            // let mut lock = data.lock().await;
            for cmd in COMMANDS.get().unwrap().iter() {
                let content = cmd.run(&ctx, &interact_cmd, &interact_cmd.data.options);
                // let content = commands::ping::run(&ctx, &interaction, &cmd.data.options);
                interact_cmd.create_interaction_response(&ctx.http, |r|
                    r.kind(model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                ).await.ok();
            }

        }
    }
}

struct BotData {
    commands: Vec<Box<dyn SlashCommand + Send + Sync>>
}

struct BotDataContainer;
impl TypeMapKey for BotDataContainer {
    type Value = Mutex<BotData>;
}

static COMMANDS: OnceLock<Vec<Box<dyn SlashCommand + Send + Sync>>> = OnceLock::new();

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("no discord token provided");
    let application_id = std::env::var("DISCORD_ID").expect("no discord id provided");
    let guild_id = GuildId(std::env::var("GUILD_ID").expect("no GUILD_ID").parse::<u64>().expect("invalid guild id"));

    let commands: Vec<Box<dyn SlashCommand + Send + Sync>> = vec![
        Box::new(PingCommand)
    ];

    COMMANDS.set(commands).ok();


    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // let bot_data = BotData {
    //     commands
    // };

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .application_id(application_id.parse::<u64>().expect("invalid client id"))
        .await
        .expect("Error creating client");

    for cmd in COMMANDS.get().unwrap().iter() {
        guild_id.create_application_command(&client.cache_and_http.http, |mut command| {
            cmd.register(command)
        }).await.expect("error registering");
    }

    {
        // let mut data = client.data.write().await;
        // data.insert::<ChatServerContainer>(manager);
        // data.insert::<BotDataContainer>(Mutex::new(bot_data));

        // let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::ping::register(command))
        // })
        //     .await;
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
