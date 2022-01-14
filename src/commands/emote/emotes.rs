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
// emotes <regex: string> (<is-case-sensitive: boolean>)

pub const COMMAND_NAME: &str = "emotes";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Lists out all the emotes the bot currently has access to")
        .create_option(|option| {
            option
                .name("regex")
                .description("The pattern to filter emotes by")
                .kind(ApplicationCommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("is-case-sensitive")
                .description(
                    "Whether or not to check for case-sensitivity (not case-sensitive by default)",
                )
                .kind(ApplicationCommandOptionType::Boolean)
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
                .interaction_response_data(|message| message.content("/emotes"))
        })
        .await
}
