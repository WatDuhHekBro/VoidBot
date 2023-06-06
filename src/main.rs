mod commands;
mod modules;
mod util;

use dotenv::dotenv;
use modules::event_handler::Handler;
use serenity::{prelude::*, utils::token};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected environment variable DISCORD_TOKEN");

    // An application ID is required to register slash commands.
    // It's usually your bot's client ID, which can be derived from your bot's token.
    let application_id: u64 = if let Ok(application_id) = env::var("DISCORD_APPLICATION_ID") {
        application_id
            .parse()
            .expect("DISCORD_APPLICATION_ID must be an integer")
    } else {
        let Some((id, _)) = token::parse(&token) else {
            panic!("The token you provided is invalid.");
        };
        id.0
    };

    // In order for the emote cache to update, the GUILD_EMOJIS_UPDATE intent must be active.
    let mut client = Client::builder(token, GatewayIntents::GUILD_EMOJIS_AND_STICKERS)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}
