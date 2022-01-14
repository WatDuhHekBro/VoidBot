use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug)]
pub enum DaylightSavingsRegion {
    NotAvailable,
    NorthAmerica,
    Europe,
    SouthernHemisphere,
}

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub timezone_offset: Option<i8>, // assume +1 is 1 hour from UTC
    pub daylight_savings_region: DaylightSavingsRegion,
}

impl User {
    fn new(id: u64) -> User {
        User {
            id,
            timezone_offset: None,
            daylight_savings_region: DaylightSavingsRegion::NotAvailable,
        }
    }

    pub fn read(db: &Connection, id: u64) -> User {
        let user = db
            .query_row(
                "SELECT ID, TimezoneOffset, DaylightSavingsRegion FROM Users WHERE ID = ?",
                [id],
                |row| {
                    let dst_code: i8 = row.get(2)?;

                    let daylight_savings_region = match dst_code {
                        1 => DaylightSavingsRegion::NorthAmerica,
                        2 => DaylightSavingsRegion::Europe,
                        3 => DaylightSavingsRegion::SouthernHemisphere,
                        _ => DaylightSavingsRegion::NotAvailable,
                    };

                    Ok(User {
                        id: row.get(0)?,
                        timezone_offset: row.get(1)?,
                        daylight_savings_region,
                    })
                },
            )
            .optional()
            .unwrap();

        if let Some(user) = user {
            user
        } else {
            User::new(id)
        }
    }

    pub fn write(self, db: &Connection) {
        let dst_code: i8 = match self.daylight_savings_region {
            DaylightSavingsRegion::NorthAmerica => 1,
            DaylightSavingsRegion::Europe => 2,
            DaylightSavingsRegion::SouthernHemisphere => 3,
            _ => 0,
        };

        db.execute(
            "INSERT INTO Users (ID, TimezoneOffset, DaylightSavingsRegion) VALUES (?, ?, ?)",
            params![self.id, self.timezone_offset, dst_code],
        )
        .unwrap();
    }
}
