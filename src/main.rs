use dotenv::dotenv;
use serenity::{client::bridge::gateway::GatewayIntents, prelude::*};
use std::env;

mod commands;
mod database;
mod modules;
use modules::event_handler::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    database::core::migrate_database();

    let token = env::var("DISCORD_TOKEN").expect("Expected environmental variable DISCORD_TOKEN");

    // I don't like having to explicitly define this, but it's a minor inconvenience anyway.
    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected environmental variable APPLICATION_ID")
        .parse()
        .expect("APPLICATION_ID must be an integer");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .intents(GatewayIntents::empty())
        .await
        .expect("Error creating client");

    client.start().await.expect("Error starting client");
}
