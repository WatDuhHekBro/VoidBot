use crate::modules::shared_data::SharedData;
use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        id::GuildId,
        prelude::{Emoji, EmojiId},
    },
    prelude::*,
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
            _ => {}
        };
    }

    // All current_state contains is the updated state of all emotes on the server, it can replace the one in the emote cache.
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

        if let Some(data) = data.get_mut::<SharedData>() {
            let emote_cache = &mut data.emote_cache;
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
        data.insert::<SharedData>(SharedData { emote_cache });

        println!("Finished creating emote cache...");
    }
}
