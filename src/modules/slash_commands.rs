use std::env;

use serenity::{
    builder::CreateApplicationCommands,
    model::{
        gateway::Ready,
        guild::Guild,
        id::{CommandId, GuildId},
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
                ApplicationCommandPermissionType,
            },
            InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    prelude::*,
};

use crate::commands;
use crate::modules::event_handler::Handler;

// Will be guild slash commands in dev and global in prod
fn set_global_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
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
        // Miscellaneous
        .create_application_command(|command| commands::config::define(command))
}

async fn set_command_permission(ctx: &Context, guild: GuildId, command: CommandId, users: &[u64]) {
    // Even if the same user ID is put, later calls to create_permission will simply override the previous calls.
    guild
        .create_application_command_permission(&ctx.http, command, |permissions| {
            for user_id in users {
                permissions.create_permission(|permission| {
                    permission
                        .id(*user_id)
                        .kind(ApplicationCommandPermissionType::User)
                        .permission(true)
                });
            }

            permissions
        })
        .await
        .expect("Error registering command permissions.");
}

impl Handler {
    // Thought Process: All user-specific permissions should be static because they're easily
    // enumerable (i.e. bot managers and server owners) while role-specific permissions should
    // be dynamic because in the case of admins, it would require you to search through all
    // users to check if they have admin permissions (i.e. admins, moderators), which the
    // server owner can assign to a role. During the loading phase, every guild will have user
    // permissions baked in.
    #[allow(unused_variables)]
    pub async fn register_slash_commands(&self, ctx: &Context, ready: &Ready) {
        // First register command definitions
        #[cfg(not(debug_assertions))]
        let commands = register_commands(ctx).await;
        #[cfg(debug_assertions)]
        let commands = register_dev_commands(ctx).await;

        // Then register command permissions
        #[cfg(not(debug_assertions))]
        register_permissions(ctx, ready, &commands).await;
        #[cfg(debug_assertions)]
        register_dev_permissions(ctx, &commands).await;
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
                commands::stream::handle(ctx, interaction, options).await
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
            // Miscellaneous
            commands::config::COMMAND_NAME => {
                let subcommand = subcommand.expect("Expected a subcommand for /config");

                match subcommand {
                    commands::config::default_voice::COMMAND_NAME => {
                        commands::config::default_voice::handle(ctx, interaction, options).await
                    }
                    commands::config::stream_embeds_channel::COMMAND_NAME => {
                        commands::config::stream_embeds_channel::handle(ctx, interaction, options)
                            .await
                    }
                    _ => reply_invalid_command(ctx, interaction).await,
                }
            }
            _ => reply_invalid_command(ctx, interaction).await,
        }
        .expect("Error replying to slash command.");
    }
}

#[allow(dead_code)]
async fn register_commands(ctx: &Context) -> Vec<ApplicationCommand> {
    ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        set_global_commands(commands)
    })
    .await
    .expect("Error on registering slash commands in production mode.")
}

#[cfg(debug_assertions)]
async fn register_dev_commands(ctx: &Context) -> Vec<ApplicationCommand> {
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
                        .expect("Each guild in DEV_CLEAR must be an integer or a wildcard (*)."),
                );

                guild
                    .set_application_commands(&ctx.http, |commands| commands)
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
            .expect("Expected environment variable DEV_GUILD")
            .parse()
            .expect("DEV_GUILD must be an integer"),
    );

    dev_guild
        .set_application_commands(&ctx.http, |commands| set_global_commands(commands))
        .await
        .expect("Error registering slash commands on dev guild.")
}

#[allow(dead_code)]
async fn register_permissions(ctx: &Context, ready: &Ready, commands: &[ApplicationCommand]) {
    let mut bot_owner: Option<u64> = None;

    if let Some(bot_owner_env) = env::var("DISCORD_BOT_OWNER").ok() {
        bot_owner = Some(
            bot_owner_env
                .parse()
                .expect("DISCORD_BOT_OWNER must be an integer"),
        );
    }

    // While the permission has to be applied to every guild, the permission is based on one global command.
    let mut config_command: Option<CommandId> = None;

    // Loop through commands and find the entry where the command is /config
    for command in commands {
        if command.name.as_str() == commands::config::COMMAND_NAME {
            config_command = Some(command.id);
        }
    }

    let config_command =
        config_command.expect("/config command not found when registering permissions.");

    // The GuildStatus vector seems to be Offline(GuildUnavailable) every time, so this fetches PartialGuilds
    for guild in &ready.guilds {
        let guild_id = guild.id();
        let guild = Guild::get(&ctx.http, guild_id).await.expect(
            "Expected all guilds to be available while fetching partial data during ready phase.",
        );
        let guild_owner = guild.owner_id.0;

        let user_ids = if let Some(bot_owner) = bot_owner {
            vec![bot_owner, guild_owner]
        } else {
            vec![guild_owner]
        };

        set_command_permission(ctx, guild_id, config_command, &user_ids[..]).await;
    }
}

#[cfg(debug_assertions)]
async fn register_dev_permissions(ctx: &Context, commands: &[ApplicationCommand]) {
    let dev_guild = GuildId(
        env::var("DEV_GUILD")
            .expect("Expected environment variable DEV_GUILD")
            .parse()
            .expect("DEV_GUILD must be an integer"),
    );

    let bot_owner: u64 = env::var("DISCORD_BOT_OWNER")
        .expect("Environment variable DISCORD_BOT_OWNER required for dev mode")
        .parse()
        .expect("DISCORD_BOT_OWNER must be an integer");

    let mut config_command: Option<CommandId> = None;

    // Loop through commands and find the entry where the command is /config
    for command in commands {
        if command.name.as_str() == commands::config::COMMAND_NAME {
            config_command = Some(command.id);
        }
    }

    set_command_permission(
        ctx,
        dev_guild,
        config_command.expect("/config command not found when registering permissions."),
        &[bot_owner],
    )
    .await;
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
