use rusqlite::{params, Connection, OptionalExtension};

pub fn get_default_name(db: &Connection, guild_id: u64, channel_id: u64) -> Option<String> {
    let channel_name = db
        .query_row(
            "SELECT ChannelName FROM DefaultVoiceChannelNames WHERE GuildID = ? AND ChannelID = ?",
            [guild_id, channel_id],
            |row| Ok(row.get(0)?),
        )
        .optional()
        .unwrap();

    if let Some(channel_name) = channel_name {
        channel_name
    } else {
        None
    }
}

pub fn set_default_name(db: &Connection, guild_id: u64, channel_id: u64, channel_name: String) {
    db.execute(
        "INSERT INTO DefaultVoiceChannelNames (GuildID, ChannelID, ChannelName) VALUES (?, ?, ?)",
        params![guild_id, channel_id, channel_name],
    )
    .unwrap();
}

pub fn remove_default_name(db: &Connection, guild_id: u64, channel_id: u64) {
    db.execute(
        "DELETE FROM DefaultVoiceChannelNames WHERE GuildID = ? AND ChannelID = ?",
        [guild_id, channel_id],
    )
    .unwrap();
}
