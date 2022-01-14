use serenity::{
    builder::CreateApplicationCommand,
    model::{
        channel::ChannelType,
        interactions::{
            application_command::{
                ApplicationCommandInteraction, ApplicationCommandInteractionDataOption,
                ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
            },
            InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    prelude::*,
};

use rusqlite::Connection;

use crate::database::{
    core::DATABASE_FILE,
    structs::{default_vc_names, guild::Guild},
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
        options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        let channel = options
            .get(0)
            .expect("Expected parameter /config default-voice <channel>")
            .resolved
            .as_ref()
            .unwrap();
        let name = options.get(1);
        let output;

        if let Some(guild_id) = interaction.guild_id {
            if let ApplicationCommandInteractionDataOptionValue::Channel(channel) = channel {
                if channel.kind == ChannelType::Voice {
                    let db = Connection::open(DATABASE_FILE).unwrap();
                    let guild_id = guild_id.0;
                    let channel_id = channel.id.0;

                    if let Some(name) = name {
                        if let ApplicationCommandInteractionDataOptionValue::String(name) =
                            name.resolved.as_ref().unwrap()
                        {
                            default_vc_names::set_default_name(
                                &db,
                                guild_id,
                                channel_id,
                                name.to_string(),
                            );
                            output = format!(
                                "Successfully set default name to `{}` for <#{}>.",
                                name, channel_id
                            );
                        } else {
                            panic!("Expected resolved /config default-voice <channel> <name>");
                        }
                    } else {
                        default_vc_names::remove_default_name(&db, guild_id, channel_id);
                        output =
                            format!("Successfully removed default name for <#{}>.", channel_id);
                    }
                } else {
                    output = String::from("You must enter a voice channel.");
                }
            } else {
                panic!("Expected resolved /config default-voice <channel>");
            }
        } else {
            output = String::from("You must use this command in a server.");
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content(output)
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
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
        options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        let channel = options.get(0);
        let output;

        if let Some(guild_id) = interaction.guild_id {
            let db = Connection::open(DATABASE_FILE).unwrap();
            let guild_id = guild_id.0;

            if let Some(channel) = channel {
                if let ApplicationCommandInteractionDataOptionValue::Channel(channel) =
                    channel.resolved.as_ref().unwrap()
                {
                    if channel.kind == ChannelType::Text {
                        let channel_id = channel.id.0;
                        let mut guild = Guild::read(&db, guild_id);
                        guild.streaming_channel = Some(channel_id);
                        guild.write(&db);
                        output = format!(
                            "Successfully set stream embeds to show up in <#{}>.",
                            channel_id
                        );
                    } else {
                        output = String::from("You must enter a text channel.");
                    }
                } else {
                    panic!("Expected resolved /config stream-embeds-channel <channel>");
                }
            } else {
                let mut guild = Guild::read(&db, guild_id);
                guild.streaming_channel = None;
                guild.write(&db);
                output = String::from("Successfully removed stream embeds from this guild.");
            }
        } else {
            output = String::from("You must use this command in a server.");
        }

        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .content(output)
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    })
            })
            .await
    }
}
