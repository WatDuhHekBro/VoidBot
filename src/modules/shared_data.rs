use serenity::{
    model::{id::GuildId, prelude::Emoji},
    prelude::TypeMapKey,
};
use std::collections::HashMap;

// Each struct defined will have its own separate data, so there's no need to create one struct with everything.
// impl TypeMapKey for NewStruct

#[derive(Debug)]
pub struct EmoteCache;

impl TypeMapKey for EmoteCache {
    type Value = HashMap<GuildId, Vec<Emoji>>;
}
