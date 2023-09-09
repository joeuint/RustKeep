// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rust_keep::structs::Database;
use rust_keep::structs::Entry;
use rust_keep::utils::database;
use rust_keep::utils::dbops;
use rust_keep::utils::generator;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn gen_passphrase(handle: tauri::AppHandle, amount: u16) -> Vec<String> {
    // NOTE: Strongbox repo has wordlists that were used here.
    // They are useful as the diceware numbers were removed
    // https://github.com/strongbox-password-safe/Strongbox/tree/master/resources/wordlists
    let wordlist_path = handle
        .path_resolver()
        .resolve_resource("resources/eff_large_wordlist.txt")
        .expect("Failed to get wordlist file");

    let text = std::fs::read_to_string(wordlist_path).unwrap();

    return generator::generate_passphrase(amount, text);
}

#[tauri::command]
fn gen_password(amount: u16) -> String {
    return generator::generate_password(amount);
}

#[tauri::command]
fn create_database(path: &str, password: &str) {
    database::create_database(path, password.as_bytes(), None);
}

#[tauri::command]
fn open_database(path: &str, password: &str) -> Result<(String, [u8; 32]), String> {
    let stringed_path = path.to_string();
    println!(
        "{:?}",
        database::read_database(&stringed_path, password.as_bytes())
    );

    return database::read_database(&stringed_path, password.as_bytes());
}

#[tauri::command]
fn add_entry(database: Database, entry: Entry) -> Database {
    return dbops::add_entry(&database, &entry);
}

#[tauri::command]
fn save_database(path: &str, database: &str, password: &str) {
    database::create_database(path, password.as_bytes(), Some(database))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_database,
            gen_passphrase,
            gen_password,
            open_database,
            add_entry,
            save_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
