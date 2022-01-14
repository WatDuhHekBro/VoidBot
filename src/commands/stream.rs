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
// stream info (<title: string>) (<description: string>) (<thumbnail: string>)
// stream set-stream-embeds-channel (<#channel>)

pub const COMMAND_NAME: &str = "stream";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("N/A")
        .create_option(|option| {
            option
                .name(info::COMMAND_NAME)
                .description("Modifies your current stream embed")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("title")
                        .description("The title of your stream embed")
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|option| {
                    option
                        .name("description")
                        .description("The description of your stream embed (supports Markdown formatting)")
                        .kind(ApplicationCommandOptionType::String)
                })
                .create_sub_option(|option| {
                    option
                        .name("thumbnail")
                        .description("The link to an image to set as the thumbnail")
                        .kind(ApplicationCommandOptionType::String)
                })
        })
        .create_option(|option| {
            option
                .name(set_stream_embeds_channel::COMMAND_NAME)
                .description("Configures the text channel to receive stream embeds (the bot must be able to send messages here)")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("channel")
                        .description("The channel to target (will disable stream embeds if empty)")
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
}

pub mod info {
    use super::*;

    pub const COMMAND_NAME: &str = "info";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/stream info"))
            })
            .await
    }
}

pub mod set_stream_embeds_channel {
    use super::*;

    pub const COMMAND_NAME: &str = "set-stream-embeds-channel";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.content("/stream set-stream-embeds-channel")
                    })
            })
            .await
    }
}
