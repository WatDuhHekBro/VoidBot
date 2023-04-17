// Although it'd be more efficient to have individual getters/setters for each column, it's just not worth the effort for this size.
// Struct::read(&db, id) - Build an object around a row in a table
// object.write(&db) - Commit changes to database and deallocate the existing reference
pub mod default_vc_names;
pub mod guild;
pub mod user;
