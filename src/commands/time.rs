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
// time show (<@user>)
// time setup
// time delete
// time utc
// time dst-info

pub const COMMAND_NAME: &str = "time";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("N/A")
        .create_option(|option| {
            option
                .name(show::COMMAND_NAME)
                .description(
                    "Display a user's current local time (or your own if no user is specified)",
                )
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .name("user")
                        .description("The user to check (if any)")
                        .kind(ApplicationCommandOptionType::User)
                })
        })
        .create_option(|option| {
            option
                .name(setup::COMMAND_NAME)
                .description("Registers your timezone info to the bot")
                .kind(ApplicationCommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name(delete::COMMAND_NAME)
                .description("Removes your timezone info from the bot")
                .kind(ApplicationCommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name(utc::COMMAND_NAME)
                .description("Displays the current time in UTC")
                .kind(ApplicationCommandOptionType::SubCommand)
        })
        .create_option(|option| {
            option
                .name(dst_info::COMMAND_NAME)
                .description("Displays the different options for configuring Daylight Savings info")
                .kind(ApplicationCommandOptionType::SubCommand)
        })
}

pub mod show {
    use super::*;

    pub const COMMAND_NAME: &str = "show";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/time show"))
            })
            .await
    }
}

pub mod setup {
    use super::*;

    pub const COMMAND_NAME: &str = "setup";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/time setup"))
            })
            .await
    }
}

pub mod delete {
    use super::*;

    pub const COMMAND_NAME: &str = "delete";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/time delete"))
            })
            .await
    }
}

pub mod utc {
    use super::*;

    pub const COMMAND_NAME: &str = "utc";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/time utc"))
            })
            .await
    }
}

pub mod dst_info {
    use super::*;

    pub const COMMAND_NAME: &str = "dst-info";

    pub async fn handle(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("/time dst-info"))
            })
            .await
    }
}
