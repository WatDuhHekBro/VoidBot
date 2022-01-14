use serenity::{
    builder::CreateApplicationCommand,
    model::interactions::{
        application_command::{
            ApplicationCommandInteraction, ApplicationCommandInteractionDataOption,
        },
        InteractionResponseType,
    },
    prelude::*,
};

/////////////////////
// Command Outline //
/////////////////////
// emote-registry ???

pub const COMMAND_NAME: &str = "emote-registry";

pub fn define(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name(COMMAND_NAME).description("???")
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
                .interaction_response_data(|message| message.content("/emote-registry"))
        })
        .await
}
