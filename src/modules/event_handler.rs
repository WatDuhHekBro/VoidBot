use crate::{modules::shared_data::EmoteCache, util::get_message_ref};
use serenity::{
    async_trait,
    model::{
        application::interaction::{Interaction, InteractionResponseType, MessageFlags},
        channel::Message,
        gateway::Ready,
        id::GuildId,
        prelude::{Emoji, EmojiId},
    },
    prelude::*,
    utils::ArgumentConvert,
};
use std::collections::HashMap;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match &interaction {
            Interaction::ApplicationCommand(interaction) => {
                self.handle_slash_commands(&ctx, interaction).await;
            }
            Interaction::MessageComponent(_) => {}
            Interaction::ModalSubmit(interaction) => {
                // Get the message info and emotes to react with
                //let c = &interaction.data.components;
                let parsed = get_message_ref(&interaction.data.custom_id); // react-query=<channel>-<message>

                // Then reply to the resolved message
                let reply = match parsed {
                    Some((channel, message)) => {
                        let message = Message::convert(
                            &ctx,
                            None,
                            Some(channel),
                            message.to_string().as_str(),
                        )
                        .await
                        .unwrap();

                        //let emote = EmojiId::from(1055589478594527345);
                        let emote = Emoji::convert(
                            &ctx,
                            //Some(GuildId::from(985682698565718086)),
                            None,
                            None,
                            "https://cdn.discordapp.com/emojis/1055589478594527345.webp",
                        )
                        .await
                        .unwrap();
                        message.react(&ctx.http, emote).await.unwrap();
                        "Reacting..."
                    }
                    None => "Failed to react to the message.",
                };

                // And then notify the user on if it was a success
                interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(reply).flags(MessageFlags::EPHEMERAL)
                            })
                    })
                    .await
                    .unwrap();
            }
            _ => {}
        };
    }

    // All current_state contains is the updated state of all emotes on the server, it can replace the one in the emote cache.
    // To avoid the gateway intent, maybe have a /refresh command to rebuild the emote cache with a global cooldown.
    async fn guild_emojis_update(
        &self,
        ctx: Context,
        guild_id: GuildId,
        current_state: HashMap<EmojiId, Emoji>,
    ) {
        // Convert the HashMap to a Vec, the key is redundant.
        let current_state: Vec<Emoji> = current_state.into_values().collect();

        // Then store the new state into the emote cache.
        let mut data = ctx.data.write().await;

        if let Some(emote_cache) = data.get_mut::<EmoteCache>() {
            emote_cache.insert(guild_id, current_state);
        } else {
            println!("Error: Failed to secure write lock on shared data while receiving an emote update!")
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Logged in as {}, ready to serve {} guilds, running {} v{}.",
            ready.user.tag(),
            ready.guilds.len(),
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );

        self.register_slash_commands(&ctx, &ready).await;

        // Serenity doesn't keep an emote cache, so it needs to be done manually.
        // Create this shared data via Client::data (https://github.com/serenity-rs/serenity/issues/945)
        // Create the initial cache during the "ready" event, then update it through the Server Members intent (GUILD_EMOJIS_UPDATE).
        println!("Creating emote cache...");

        // According to the description of EventHandler::cache_ready(), it'd be better for dealing with the cache,
        // but the problem is that the function is never actually called for some reason. Using EventHandler::ready()
        // seems to be accurate enough at least.
        let guilds = ctx.cache.guilds();
        let mut emote_cache = HashMap::new();

        for guild in guilds {
            let emotes = guild.emojis(&ctx.http).await;

            if let Ok(emotes) = emotes {
                emote_cache.insert(guild, emotes);
            } else {
                println!("Error: Failed to retrieve emotes for guild {guild}.");
            }
        }

        let mut data = ctx.data.write().await;
        data.insert::<EmoteCache>(emote_cache);

        println!("Finished creating emote cache...");
    }
}
