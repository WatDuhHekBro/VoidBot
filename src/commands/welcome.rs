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

pub const COMMAND_NAME: &str = "welcome";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Welcome a user")
        .create_option(|option| {
            option
                .name("lmao")
                .description("git rekt")
                .kind(ApplicationCommandOptionType::SubCommand)
                .create_sub_option(|option| {
                    option
                        .kind(ApplicationCommandOptionType::User)
                        .name("user")
                        .description("not a bot")
                })
        })
        .create_option(|option| {
            option
                .name("group")
                .description("sample text")
                .kind(ApplicationCommandOptionType::SubCommandGroup)
                .create_sub_option(|option| {
                    option
                        .name("fah")
                        .description("rohdah")
                        .kind(ApplicationCommandOptionType::SubCommand)
                })
        })
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &Vec<ApplicationCommandInteractionDataOption>,
) -> Result<(), serenity::Error> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("<:asdf:798534948679843840>"))
        })
        .await
}

pub mod subcommands {
    use super::*;

    pub async fn yeet(
        ctx: &Context,
        interaction: &ApplicationCommandInteraction,
        _options: &Vec<ApplicationCommandInteractionDataOption>,
    ) -> Result<(), serenity::Error> {
        interaction
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("yeet"))
            })
            .await
    }
}
