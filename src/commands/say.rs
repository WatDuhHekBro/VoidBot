use serenity::{
    builder::CreateApplicationCommand,
    model::application::{
        command::CommandOptionType,
        interaction::{
            application_command::{ApplicationCommandInteraction, CommandDataOption},
            InteractionResponseType,
        },
    },
    prelude::*,
};

/////////////////////
// Command Outline //
/////////////////////
// say <message: string>
// Create commands to delete/edit messages
// Create some way to confirm your message without breaking continuity of who used /say, /check-emotes?

pub const COMMAND_NAME: &str = "say";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name(COMMAND_NAME)
        .description("Sends a message in your place with any emote you specify in /slashes/")
        .create_option(|option| {
            option
                .name("message")
                .description(
                    r#"The message to parse ("//" = slash, "\" = new line, "\\" = backslash)"#,
                )
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn handle(
    ctx: &Context,
    interaction: &ApplicationCommandInteraction,
    _options: &[CommandDataOption],
) -> Result<(), serenity::Error> {
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.content("asdf <:asdf:992812937876082748> <:asdf:1055589478594527345> <a:asdf:754077074951766280> <a:asdf:687071462154174663> <a:asdf:834292642158346271> <a:asdf:1055591264592072705>")
                })
        })
        .await
}
