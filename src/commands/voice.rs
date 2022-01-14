use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOption,
            ApplicationCommandOptionType,
        },
        InteractionResponseType,
    },
    prelude::*,
};

/////////////////////
// Command Outline //
/////////////////////
// voice (<name: string>)

pub const COMMAND_NAME: &str = "voice";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Changes the name of the current voice channel you're in")
        .create_option(|option| {
            option
                .name("name")
                .description("The channel name to change to (resets to the default if empty)")
                .kind(ApplicationCommandOptionType::String)
        })
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &Vec<ApplicationCommandInteractionDataOption>,
) -> Result<(), serenity::Error> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("/voice"))
        })
        .await
}
