use futures::try_join;
use std::env;

use serenity::{
    builder::CreateApplicationCommands,
    model::{
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteraction,
                ApplicationCommandInteractionDataOption, ApplicationCommandOptionType,
            },
            Interaction, InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
        },
    },
    prelude::*,
};

use crate::commands;
use crate::modules::event_handler::Handler;

impl Handler {
    pub async fn register_slash_commands(&self, ctx: &Context) {
        let mut dev_guild: Option<GuildId> = None;

        // Check if it's dev mode and if a valid dev guild is specified
        if env::var("DEV").is_ok() {
            if let Ok(raw_dev_guild) = env::var("DEV_GUILD") {
                dev_guild = Some(GuildId(
                    raw_dev_guild.parse().expect("DEV_GUILD must be an integer"),
                ));
            }
        }

        // Then check if there's a valid dev guild, otherwise set it globally
        if let Some(dev_guild) = dev_guild {
            println!("Registered slash commands in dev mode.");

            GuildId::set_application_commands(&dev_guild, &ctx.http, |commands| {
                set_dev_guild_commands(set_normal_commands(commands))
            })
            .await
            .expect("Error registering slash commands on dev guild");
        } else {
            println!("Registered slash commands in release mode.");

            try_join!(
                ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
                    set_normal_commands(commands)
                }),
                set_guild_commands(&ctx)
            )
            .expect("Error on registering slash commands in release mode.");
        }
    }

    pub async fn handle_slash_commands(&self, ctx: &Context, interaction: &Interaction) {
        if let Interaction::ApplicationCommand(interaction) = interaction {
            let command = interaction.data.name.as_str();
            let mut subcommand: Option<&str> = None;
            let mut subcommand_group: Option<&str> = None;
            let mut options: &Vec<ApplicationCommandInteractionDataOption> =
                &interaction.data.options;

            // Gather subcommand and subcommand group info if it exists
            if let Some(option) = interaction.data.options.get(0) {
                if option.kind == ApplicationCommandOptionType::SubCommand {
                    subcommand = Some(option.name.as_str());
                    options = &option.options;
                } else if option.kind == ApplicationCommandOptionType::SubCommandGroup {
                    subcommand_group = Some(option.name.as_str());
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
                commands::welcome::COMMAND_NAME => {
                    if let Some(subcommand) = subcommand {
                        match subcommand {
                            "lmao" => {
                                commands::welcome::subcommands::yeet(&ctx, &interaction, &options)
                                    .await
                            }
                            _ => reply_invalid_command(&ctx, &interaction).await,
                        }
                    } else {
                        commands::welcome::handle(&ctx, &interaction, &options).await
                    }
                }
                _ => reply_invalid_command(&ctx, &interaction).await,
            }
            .expect("Error replying to slash command");
        }
    }
}

// Will be guild slash commands in dev and global in prod
fn set_normal_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|command| commands::welcome::define(command))
}

// Will always be guild-specific slash commands, but the guild depends on release values
async fn set_guild_commands(
    ctx: &Context,
) -> Result<(Vec<ApplicationCommand>, Vec<ApplicationCommand>), serenity::Error> {
    try_join!(
        GuildId::set_application_commands(&GuildId(0u64), &ctx.http, |commands| {
            commands.create_application_command(|command| command)
        }),
        GuildId::set_application_commands(&GuildId(0u64), &ctx.http, |commands| {
            commands.create_application_command(|command| command)
        })
    )
}

// Same as above, but the provided dev guild is used
fn set_dev_guild_commands(
    commands: &mut CreateApplicationCommands,
) -> &mut CreateApplicationCommands {
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
            .interaction_response_data(|message| message.content("**Error:** Invalid command name! This probably means that the command definitions haven't been updated yet or there's a glaring oversight in the code.").flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL))
    })
    .await
}
