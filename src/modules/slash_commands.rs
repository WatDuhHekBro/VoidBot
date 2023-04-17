use std::env;

use serenity::{
    builder::CreateApplicationCommands,
    model::{
        application::{
            command::{Command, CommandOptionType},
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
                MessageFlags,
            },
        },
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};

use crate::commands;
use crate::modules::event_handler::Handler;

// Will be guild slash commands in dev and global in prod
fn set_global_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        // Emote Commands
        .create_application_command(|command| commands::say::define(command))
        .create_application_command(|command| commands::react::define(command))
        .create_application_command(|command| commands::emotes::define(command))
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
    }

    pub async fn handle_slash_commands(
        &self,
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
    ) {
        let command = interaction.data.name.as_str();
        //let mut subcommand: Option<&str> = None;
        let mut _subcommand_group: Option<&str> = None;
        let mut options = &interaction.data.options;

        // Gather subcommand and subcommand group info if it exists
        if let Some(option) = interaction.data.options.get(0) {
            if option.kind == CommandOptionType::SubCommand {
                //subcommand = Some(option.name.as_str());
                options = &option.options;
            } else if option.kind == CommandOptionType::SubCommandGroup {
                _subcommand_group = Some(option.name.as_str());
                let option = option
                    .options
                    .get(0)
                    .expect("Subcommand Groups must have exactly one nested option");

                if option.kind == CommandOptionType::SubCommand {
                    //subcommand = Some(option.name.as_str());
                    options = &option.options;
                } else {
                    panic!("The option of a subcommand group isn't a subcommand");
                }
            }
        }

        // Find the appropriate handler then execute it
        match command {
            // Emote Commands
            commands::say::COMMAND_NAME => commands::say::handle(ctx, interaction, options).await,
            commands::react::COMMAND_NAME => {
                commands::react::handle(ctx, interaction, options).await
            }
            commands::emotes::COMMAND_NAME => {
                commands::emotes::handle(ctx, interaction, options).await
            }
            _ => reply_invalid_command(ctx, interaction).await,
        }
        .expect("Error replying to slash command.");
    }
}

#[allow(dead_code)]
async fn register_commands(ctx: &Context) -> Vec<Command> {
    Command::set_global_application_commands(&ctx.http, |commands| set_global_commands(commands))
        .await
        .expect("Error on registering slash commands in production mode.")
}

#[cfg(debug_assertions)]
async fn register_dev_commands(ctx: &Context) -> Vec<Command> {
    // Clear existing slash commands based on the DEV_CLEAR environment variable
    // This should be called before a potential error for DEV_GUILD so you can clear your dev instance once you've finished up your changes
    if let Ok(guilds) = env::var("DEV_CLEAR") {
        let guilds: Vec<&str> = guilds.split(',').collect();

        for guild in guilds {
            if guild == "*" {
                Command::set_global_application_commands(&ctx.http, |commands| commands)
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

    let commands = dev_guild
        .set_application_commands(&ctx.http, |commands| set_global_commands(commands))
        .await
        .expect("Error registering slash commands on dev guild.");

    println!("Registered slash commands in dev mode.");

    commands
}

async fn reply_invalid_command(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    interaction
    .create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message
                    .content("**Error:** Invalid command name! This probably means that the command definitions haven't been updated yet or there's a glaring oversight in the code.")
                    .flags(MessageFlags::EPHEMERAL)
            })
    })
    .await
}
