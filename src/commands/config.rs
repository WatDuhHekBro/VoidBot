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

// Note: Because of the way Discord's built-in slash command permissions work, permissions are
// command-wide, so it's better to group all restricted configuration commands into one place
// rather than trying to have runtime command checks.
// Note: Because permissions are guild-specific, setting the default permission to false
// essentially makes a command guild-only.
// Note: /config will be available to the specified bot owner and guild owners.

/////////////////////
// Command Outline //
/////////////////////
// config default-voice <#channel> (<name: string>)
// config stream-embeds-channel (<#channel>)

pub const COMMAND_NAME: &str = "config";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("N/A")
        .default_permission(false)
        .create_option(|option| {
            option
                .name(default_voice::COMMAND_NAME)
                .description("Sets the default name for a voice channel")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("channel")
                        .description("The voice channel to target")
                        .kind(ApplicationCommandOptionType::Channel)
                        .required(true)
                })
                .create_sub_option(|option| {
                    option
                        .name("name")
                        .description("The channel name to reset to (removes default channel name if empty)")
                        .kind(ApplicationCommandOptionType::String)
                })
        })
        .create_option(|option| {
            option
                .name(stream_embeds_channel::COMMAND_NAME)
                .description("Configures a text channel to receive stream embeds (the bot must be able to send messages here)")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("channel")
                        .description("The channel to target (will disable stream embeds for this guild if empty)")
                        .kind(ApplicationCommandOptionType::Channel)
                })
        })
}

pub mod default_voice {
    use super::*;

    pub const COMMAND_NAME: &str = "default-voice";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/config default-voice"))
            })
            .await
    }
}

pub mod stream_embeds_channel {
    use super::*;

    pub const COMMAND_NAME: &str = "stream-embeds-channel";

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
                        message.content("/config stream-embeds-channel")
                    })
            })
            .await
    }
}
