use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::interaction::*;
use serenity::prelude::*;

pub struct PingCommand;
impl SlashCommand for PingCommand {
    fn run(&mut self, ctx: &Context, interact: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String {
        format!("Hey, {}", interact.user.name).to_string()
    }

    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}

pub trait SlashCommand {
    fn run(&mut self, ctx: &Context, interact: &ApplicationCommandInteraction, options: &[CommandDataOption]) -> String;

    fn register<'a>(&'a self, command: &'a mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}