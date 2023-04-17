use serenity::{
    builder::CreateApplicationCommand,
    model::application::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOption},
            InteractionResponseType,
        },
    },
    prelude::*,
};

use serenity::cache::Cache;

/////////////////////
// Command Outline //
/////////////////////
// emotes (<pattern: string>) (<is-case-sensitive: boolean>)

pub const COMMAND_NAME: &str = "emotes";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Lists out all the emotes the bot currently has access to")
        .create_option(|option| {
            option
                .name("pattern")
                .description("The regex pattern to filter emotes by")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("is-case-sensitive")
                .description(
                    "Whether or not to check for case-sensitivity (not case-sensitive by default)",
                )
                .kind(CommandOptionType::Boolean)
        })
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    options: &Vec<CommandDataOption>,
) -> Result<(), serenity::Error> {
    let a = ctx.cache.guilds();
    println!("{options:?}");
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("/emotes"))
        })
        .await
}
