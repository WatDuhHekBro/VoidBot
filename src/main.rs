mod commands;
mod modules;

use dotenv::dotenv;
use modules::event_handler::Handler;
use serenity::prelude::*;
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
        let index = token
            .find('.')
            .expect("A proper bot token must consist of three parts separated by periods.");
        let client_id = &token[..index];
        let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
        let client_id = base64::decode_config(client_id, base64_config).unwrap();
        std::str::from_utf8(&client_id)
            .expect("Expected decoded token slice to be UTF-8.")
            .parse()
            .expect("Expected decoded token slice to be an integer.")
    };

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}
