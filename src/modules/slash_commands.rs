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
            Interaction, InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
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

    pub async fn handle_slash_commands(&self, ctx: &Context, interaction: &Interaction) {
        if let Interaction::ApplicationCommand(interaction) = interaction {
            let command = interaction.data.name.as_str();
            let mut subcommand: Option<&str> = None;
            let mut subcommand_group: Option<&str> = None;
            let mut options = &interaction.data.options;

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

#[allow(dead_code)]
async fn register_commands(ctx: &Context) {
    ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
        set_normal_commands(commands)
    })
    .await
    .expect("Error on registering slash commands in production mode.");

    println!("Registered slash commands in production mode.");
}

#[cfg(debug_assertions)]
async fn register_dev_commands(ctx: &Context) {
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
    .expect("Error registering slash commands on dev guild");

    println!("Registered slash commands in dev mode.");
}

// Will be guild slash commands in dev and global in prod
fn set_normal_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands.create_application_command(|command| commands::welcome::define(command))
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
