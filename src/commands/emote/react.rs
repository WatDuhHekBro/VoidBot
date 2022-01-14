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
// react <emotes: string> (<target: string>)

pub const COMMAND_NAME: &str = "react";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Reacts to the targeted message with any emotes the bot currently has access to")
        .create_option(|option| {
            option
                .name("emotes")
                .description("The list of space-separated emote names to react with")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("target")
                .description("The message to target (distance / message ID / channel-message ID pair / message link)")
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
                .interaction_response_data(|message| message.content("/react"))
        })
        .await
}
