use std::fmt::Debug;

// TODO: The data format itself should be solid and future-proof.
// I'm just not sure what I want the interface to be yet.

// set_emote, get_emote, remove_emote

#[derive(Debug)]
pub struct EmoteRegistry {
    pub user_id: u64,
    pub emote_id: u64,
    pub emote_name: String,
    pub is_animated: bool,
}
