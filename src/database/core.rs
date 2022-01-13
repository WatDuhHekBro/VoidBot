use rusqlite::Connection;

// This section will serve as the documentation for the database, because in order to guarantee
// that a database created now will have the same structure as a database that has been migrated
// through different versions, a new database starts at version one and goes through the same
// migration process. Creating separate statements for migrations and creating a new database will
// allow for some dangerous out of sync definitions. For example, version 9 via migration might
// have a column that forgot to be dropped while version 9 via creation won't include that column,
// so when someone tries to use an INSERT statement, it'll throw an error because of discrepancies.

// -=[ Current Schema ]=-
// Users
//     ID
//     TimezoneOffset (INT NULLABLE)
//     DaylightSavingsRegion (INT OPTIONAL)
// Guilds
//     ID
//     StreamingChannel (INT NULLABLE)
// DefaultVoiceChannelNames
//     GuildID
//     ChannelID
//     ChannelName (TEXT)
// EmoteRegistry
//     UserID
//     EmoteID
//     EmoteName (TEXT)
//     Animated (BOOL OPTIONAL)

// -=[ Notes ]=-
// - Unless otherwise directed above (i.e. NULLABLE / OPTIONAL), assume the "NOT NULL" constraint.
// - IDs use the "ON CONFLICT REPLACE" constraint to enable implicit UPSERT statements.
// - Any datetime stuff (marked as TIME) will be stored as a UNIX timestamp in seconds (INT).
// - Booleans (marked as BOOL) will be stored as an integer, either 0 or 1 (though it just checks for 0).

#[cfg(not(debug_assertions))]
pub const DATABASE_FILE: &str = "main.db";
#[cfg(debug_assertions)]
pub const DATABASE_FILE: &str = "test.db";

pub fn migrate_database() {
    // Because the function to migrate the database is only called once, the migrations array should also be unallocated once migrations take place.
    // Migration naming convention: 5.sql means "run this script for databases on version 5"
    // Note: Once a migration is written, DO NOT change that migration or it'll break all future migrations.
    let migrations = [include_str!("migrations/0.sql")];

    let db = Connection::open(DATABASE_FILE).unwrap();
    // The user_version pragma is limited from 0x0 to 0x7FFF_FFFF
    let mut version: u32 = db
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .expect("Error retrieving user_version pragma");

    // You only want to enumerate over the migrations you have to use
    // Will panic if the version isn't between 0 and migrations.len()
    // Since this breaks backwards compatibility, if you want to run an older version, you have to downgrade the database manually
    let selection = &migrations[version.try_into().unwrap()..migrations.len()];

    for migration_script in selection {
        db.execute_batch(migration_script).expect(&format!(
            "Invalid SQL script for migration targeting database version {}",
            version
        ));
        version += 1;
    }

    // Skip the pragma update if the database is already at the latest version
    if selection.len() != 0 {
        db.pragma_update(None, "user_version", version)
            .expect("Error updating user_version pragma");
    }
}
