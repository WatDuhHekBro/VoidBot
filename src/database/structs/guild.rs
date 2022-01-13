use rusqlite::{params, Connection, OptionalExtension};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Guild {
    pub id: u64,
    pub streaming_channel: Option<u64>,
}

impl Guild {
    fn new(id: u64) -> Guild {
        Guild {
            id,
            streaming_channel: None,
        }
    }

    pub fn read(db: &Connection, id: u64) -> Guild {
        let guild = db
            .query_row(
                "SELECT ID, StreamingChannel FROM Guilds WHERE ID = ?",
                [id],
                |row| {
                    Ok(Guild {
                        id: row.get(0)?,
                        streaming_channel: row.get(1)?,
                    })
                },
            )
            .optional()
            .unwrap();

        if let Some(guild) = guild {
            guild
        } else {
            Guild::new(id)
        }
    }

    pub fn write(self, db: &Connection) {
        db.execute(
            "INSERT INTO Guilds (ID, StreamingChannel) VALUES (?, ?)",
            params![self.id, self.streaming_channel],
        )
        .unwrap();
    }
}
