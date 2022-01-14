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
// default-voice <#channel> (<name: string>)

pub const COMMAND_NAME: &str = "default-voice";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Sets the default name for a voice channel")
        .create_option(|option| {
            option
                .name("channel")
                .description("The voice channel to target")
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("name")
                .description("The channel name to reset to (removes default channel name if empty)")
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
                .interaction_response_data(|message| message.content("/default-voice"))
        })
        .await
}
