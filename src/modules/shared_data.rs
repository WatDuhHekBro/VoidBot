use serenity::{
    model::{id::GuildId, prelude::Emoji},
    prelude::TypeMapKey,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct SharedData {
    pub emote_cache: HashMap<GuildId, Vec<Emoji>>,
}

impl TypeMapKey for SharedData {
    type Value = SharedData;
}
