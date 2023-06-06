use crate::modules::shared_data::EmoteCache;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::{
            command::CommandOptionType,
            component::InputTextStyle,
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOption},
                InteractionResponseType, MessageFlags,
            },
        },
        prelude::command::CommandType,
    },
    prelude::*,
};

/////////////////////
// Command Outline //
/////////////////////
// react <emotes: string> (<target: string>)
// Have "confirm" boolean option to make message ephemeral and show emotes to react with before reacting with it?

pub const COMMAND_NAME: &str = "react";
pub const MESSAGE_MENU_NAME: &str = "React with Emotes";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Reacts to the targeted message with any emotes the bot currently has access to")
        .create_option(|option| {
            option
                .name("emotes")
                .description("The list of space-separated emote names to react with")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("target")
                .description("The message to target (distance / message ID / channel-message ID pair / message link)")
                .kind(CommandOptionType::String)
        })
}

pub fn define_menu(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name(MESSAGE_MENU_NAME).kind(CommandType::Message)
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> Result<(), serenity::Error> {
    let data = ctx.data.read().await;

    if let Some(emote_cache) = data.get::<EmoteCache>() {
        let a = emote_cache;
    }

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("/react"))
        })
        .await
}

pub async fn handle_menu(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> Result<(), serenity::Error> {
    let message_id = format!(
        "{}-{}",
        interaction.channel_id,
        // As long as this interaction stems from a message context menu, target_id should always be the message ID.
        interaction.data.target_id.unwrap()
    );

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::Modal)
                .interaction_response_data(|message| {
                    message
                        .title("Enter the emotes to react with")
                        .flags(MessageFlags::EPHEMERAL)
                        .custom_id(format!("react-query={message_id}"))
                        .components(|components| {
                            components.create_action_row(|row| {
                                row.create_input_text(|input| {
                                    input
                                        .custom_id("react-query-input")
                                        .style(InputTextStyle::Short)
                                        .label("Emote Names")
                                        .min_length(1)
                                        .placeholder("emote1 emote2 ...")
                                        .required(true)
                                })
                            })
                        })
                })
        })
        .await
}
