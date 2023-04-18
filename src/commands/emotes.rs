use crate::modules::shared_data::EmoteCache;
use regex::Regex;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::{
            command::CommandOptionType,
            interaction::{
                application_command::{
                    ApplicationCommandInteraction, CommandDataOption,
                    CommandDataOptionValue as CmdType,
                },
                InteractionResponseType,
            },
        },
        prelude::{Emoji, Message},
    },
    prelude::*,
};

/////////////////////
// Command Outline //
/////////////////////
// emotes (<regex: string>) (<is-case-sensitive: boolean>)

pub const COMMAND_NAME: &str = "emotes";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Lists out all the emotes the bot currently has access to")
        .create_option(|option| {
            option
                .name("regex")
                .description("The regex pattern to filter emotes by")
                .kind(CommandOptionType::String)
        })
        .create_option(|option| {
            option
                .name("is-case-sensitive")
                .description(
                    "Whether or not to check the pattern for case-sensitivity (false by default)",
                )
                .kind(CommandOptionType::Boolean)
        })
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &Vec<CommandDataOption>,
) -> Result<(), serenity::Error> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|message| message.ephemeral(true))
        })
        .await
}

pub async fn handle_deferred(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    options: &Vec<CommandDataOption>,
) -> Result<Message, serenity::Error> {
    let data = ctx.data.read().await;
    let mut pattern: Option<&String> = None;
    let mut is_case_sensitive = false;

    for option in options {
        match option.name.as_str() {
            "regex" => {
                if let Some(value) = &option.resolved {
                    if let CmdType::String(value) = value {
                        pattern = Some(value);
                    }
                }
            }
            "is-case-sensitive" => {
                if let Some(value) = &option.resolved {
                    if let CmdType::Boolean(value) = value {
                        is_case_sensitive = *value;
                    }
                }
            }
            _ => {}
        }
    }

    interaction
        .create_followup_message(&ctx.http, |message| {
            if let Some(emote_cache) = data.get::<EmoteCache>() {
                // Setup regex pattern matching
                // According to the regex module's documentation, it has a worst-case linear time search, avoiding exponential blowup.
                let mut regex = None;

                if let Some(pattern) = pattern {
                    // The case sensitivity option only affects whether (?i) is present at the start of the string.
                    let compiled_regex = {
                        match is_case_sensitive {
                            true => {
                                Regex::new(pattern)
                            }
                            false => {
                                Regex::new(&format!("(?i){pattern}"))
                            }
                        }
                    };

                    if let Ok(compiled_regex) = compiled_regex {
                        regex = Some(compiled_regex);
                    } else {
                        return message.content("The regex pattern you provided was not valid.");
                    }
                }

                // Loop through the emote cache, and for each emote, only include it if it passes the pattern
                let mut emotes = Vec::<&Emoji>::new();

                if let Some(regex) = &regex {
                    for (_, cached_emotes) in emote_cache {
                        for emote in cached_emotes {
                            if regex.is_match(&emote.name) {
                                emotes.push(emote);
                            }
                        }
                    }
                } else {
                    for (_, cached_emotes) in emote_cache {
                        for emote in cached_emotes {
                            emotes.push(emote);
                        }
                    }
                }

                // Then sort the array alphabetically
                emotes.sort_by(|a, b| a.name.cmp(&b.name));

                // TODO: https://github.com/serenity-rs/serenity/blob/current/examples/e17_message_components/src/main.rs
                message.content("/emotes").components(
                    |components| components.create_action_row(
                        |row| row.create_button(
                            |button| button.custom_id("yeet").label("⬅️")
                        )
                    )
                )
            } else {
                message.content("The bot failed to secure the lock on the emote cache. Please contact the developer (the emote cache might not have been initialized).")
            }
        })
        .await
}
