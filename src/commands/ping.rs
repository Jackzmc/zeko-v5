use std::time::{Duration, Instant};
use serenity::builder::CreateApplicationCommand;
use serenity::http::CacheHttp;
use serenity::{async_trait, model};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::interaction::*;
use serenity::prelude::*;

pub struct PingCommand;
#[async_trait]
impl SlashCommand for PingCommand {
    async fn run(&self, ctx: &Context, interact: &ApplicationCommandInteraction, options: &[CommandDataOption]) {
        let now = Instant::now();
        interact.create_interaction_response(&ctx.http, |response|
            response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Calculating ping..."))
        ).await.ok();

        interact.edit_original_interaction_response(&ctx.http, |response|
            response.content(format!("Ping: {} ms", now.elapsed().as_millis()))
        ).await.ok();
    }

    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("PONG")
    }
}

#[async_trait]
pub trait SlashCommand {
    async fn run(&self, ctx: &Context, interact: &ApplicationCommandInteraction, options: &[CommandDataOption]);
    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}