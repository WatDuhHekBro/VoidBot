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
// stream (<title: string>) (<description: string>) (<thumbnail: string>)

pub const COMMAND_NAME: &str = "stream";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Modifies your current stream embed")
        .create_option(|option| {
            option
                .name("title")
                .description("The title of your stream embed")
                .kind(ApplicationCommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("description")
                .description("The description of your stream embed (supports Markdown formatting)")
                .kind(ApplicationCommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("thumbnail")
                .description("The link to an image to set as the thumbnail")
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
                .interaction_response_data(|message| message.content("/stream"))
        })
        .await
}
