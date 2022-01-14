#[cfg(debug_assertions)]
use std::env;

use serenity::{
    builder::CreateApplicationCommands,
    model::{
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
            },
            InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    prelude::*,
};

use crate::commands;
use crate::modules::event_handler::Handler;

impl Handler {
    pub async fn register_slash_commands(&self, ctx: &Context) {
        #[cfg(not(debug_assertions))]
        register_commands(&ctx).await;
        #[cfg(debug_assertions)]
        register_dev_commands(&ctx).await;
    }

    pub async fn handle_slash_commands(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) {
        let command = interaction.data.name.as_str();
        let mut subcommand: Option<&str> = None;
        let mut _subcommand_group: Option<&str> = None;
        let mut options = &interaction.data.options;

        // Gather subcommand and subcommand group info if it exists
        if let Some(option) = interaction.data.options.get(0) {
            if option.kind == ApplicationCommandOptionType::SubCommand {
                subcommand = Some(option.name.as_str());
                options = &option.options;
            } else if option.kind == ApplicationCommandOptionType::SubCommandGroup {
                _subcommand_group = Some(option.name.as_str());
                let option = option
                    .options
                    .get(0)
                    .expect("Subcommand Groups must have exactly one nested option");

                if option.kind == ApplicationCommandOptionType::SubCommand {
                    subcommand = Some(option.name.as_str());
                    options = &option.options;
                } else {
                    panic!("The option of a subcommand group isn't a subcommand");
                }
            }
        }

        // Find the appropriate handler then execute it
        match command {
            // Emote Commands
            commands::emote::say::COMMAND_NAME => {
                commands::emote::say::handle(ctx, interaction, options).await
            }
            commands::emote::react::COMMAND_NAME => {
                commands::emote::react::handle(ctx, interaction, options).await
            }
            commands::emote::emotes::COMMAND_NAME => {
                commands::emote::emotes::handle(ctx, interaction, options).await
            }
            commands::emote::emote_registry::COMMAND_NAME => {
                commands::emote::emote_registry::handle(ctx, interaction, options).await
            }
            // Stream Embeds
            commands::stream::COMMAND_NAME => {
                let subcommand = subcommand.expect("Expected a subcommand for /stream");

                match subcommand {
                    commands::stream::info::COMMAND_NAME => {
                        commands::stream::info::handle(ctx, interaction, options).await
                    }
                    commands::stream::set_stream_embeds_channel::COMMAND_NAME => {
                        commands::stream::set_stream_embeds_channel::handle(
                            ctx,
                            interaction,
                            options,
                        )
                        .await
                    }
                    _ => reply_invalid_command(ctx, interaction).await,
                }
            }
            // Timezone Info
            commands::time::COMMAND_NAME => {
                let subcommand = subcommand.expect("Expected a subcommand for /time");

                match subcommand {
                    commands::time::show::COMMAND_NAME => {
                        commands::time::show::handle(ctx, interaction, options).await
                    }
                    commands::time::setup::COMMAND_NAME => {
                        commands::time::setup::handle(ctx, interaction, options).await
                    }
                    commands::time::delete::COMMAND_NAME => {
                        commands::time::delete::handle(ctx, interaction, options).await
                    }
                    commands::time::utc::COMMAND_NAME => {
                        commands::time::utc::handle(ctx, interaction, options).await
                    }
                    commands::time::dst_info::COMMAND_NAME => {
                        commands::time::dst_info::handle(ctx, interaction, options).await
                    }
                    _ => reply_invalid_command(ctx, interaction).await,
                }
            }
            // Voice Channel Renaming
            commands::voice::COMMAND_NAME => {
                commands::voice::handle(ctx, interaction, options).await
            }
            commands::default_voice::COMMAND_NAME => {
                commands::default_voice::handle(ctx, interaction, options).await
            }
            _ => reply_invalid_command(ctx, interaction).await,
        }
        .expect("Error replying to slash command.");
    }
}

#[allow(dead_code)]
async fn register_commands(ctx: &Context) {
    ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        set_normal_commands(commands)
    })
    .await
    .expect("Error on registering slash commands in production mode.");
}

#[cfg(debug_assertions)]
async fn register_dev_commands(ctx: &Context) {
    // Clear existing slash commands based on the DEV_CLEAR environment variable
    // This should be called before a potential error for DEV_GUILD so you can clear your dev instance once you've finished up your changes
    if let Ok(guilds) = env::var("DEV_CLEAR") {
        let guilds: Vec<&str> = guilds.split(',').collect();

        for guild in guilds {
            if guild == "*" {
                ApplicationCommand::set_global_application_commands(&ctx.http, |commands| commands)
                    .await
                    .expect("Error on clearing global slash commands.");

                println!("Cleared global slash commands.");
            } else {
                let guild = GuildId(
                    guild
                        .parse()
                        .expect("Each guild in DEV_CLEAR must be an integer."),
                );

                GuildId::set_application_commands(&guild, &ctx.http, |commands| commands)
                    .await
                    .expect(&format!(
                        "Error on clearing guild slash commands for {}.",
                        guild
                    ));

                println!("Cleared guild slash commands for {}.", guild);
            }
        }
    }

    let dev_guild = GuildId(
        env::var("DEV_GUILD")
            .expect("Expected environmental variable DEV_GUILD")
            .parse()
            .expect("DEV_GUILD must be an integer"),
    );

    GuildId::set_application_commands(&dev_guild, &ctx.http, |commands| {
        set_normal_commands(commands)
    })
    .await
    .expect("Error registering slash commands on dev guild.");

    println!("Registered slash commands in dev mode.");
}

// Will be guild slash commands in dev and global in prod
fn set_normal_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        // Emote Commands
        .create_application_command(|command| commands::emote::say::define(command))
        .create_application_command(|command| commands::emote::react::define(command))
        .create_application_command(|command| commands::emote::emotes::define(command))
        .create_application_command(|command| commands::emote::emote_registry::define(command))
        // Stream Embeds
        .create_application_command(|command| commands::stream::define(command))
        // Timezone Info
        .create_application_command(|command| commands::time::define(command))
        // Voice Channel Renaming
        .create_application_command(|command| commands::voice::define(command))
        .create_application_command(|command| commands::default_voice::define(command))
}

async fn reply_invalid_command(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    interaction
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("**Error:** Invalid command name! This probably means that the command definitions haven't been updated yet or there's a glaring oversight in the code.").flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL))
    })
    .await
}
