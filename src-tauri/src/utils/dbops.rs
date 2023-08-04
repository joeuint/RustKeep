use crate::structs::{Database, Entry};

pub fn add_entry(database: &Database, entry: &Entry) -> Database {
    let mut database_clone = database.clone();
    let entry_clone = entry.clone();

    database_clone.root.entries.append(&mut vec![entry_clone]);

    return database_clone;
}
