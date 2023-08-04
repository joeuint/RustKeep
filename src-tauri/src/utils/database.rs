use super::crypto::{decrypt_data, encrypt_data};
use super::random::secure_random_bytes;
use crate::structs::Database;
use argon2::{Algorithm, ParamsBuilder, Version};
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Write};
use std::str;

// TODO: We have to migrate pretty much every panic to "actual" error handling!

const SKELETON_ENCRYPTION_DATA: &str = r#"{"version": "0.1","root": {"entries": []}}"#;
// This represents the string "RUSTKEEP" in hex bytes
const DATABASE_WATERMARK: [u8; 8] = [0x52, 0x55, 0x53, 0x54, 0x4B, 0x45, 0x45, 0x50];
const MIN_SIZE: usize = 52;

pub fn create_database(path: String, password: &[u8]) {
    // Open file safely and panic if it alerady exists
    // TODO: Show error message instead of outright panicing!
    let file_result = OpenOptions::new().write(true).create_new(true).open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => panic!("File already exists"),
            other_error => {
                panic!("Failed to open file: {:?}", other_error);
            }
        },
    };

    let mut bytes: Vec<u8> = vec![];

    // Using some sane pre defined settings
    // TODO: Add a way to tweak these
    // It should also be stored in RustKeep Databases
    // TODO: Also put this in a seperate function for consistancy!
    let mut binding = ParamsBuilder::new();
    let hasher = match binding
        .m_cost(512 * 1024)
        .output_len(32)
        .p_cost(4)
        .t_cost(2)
        .context(Algorithm::Argon2id, Version::V0x13)
    {
        Ok(context) => context,
        Err(error) => panic!("Failed to save params {:?}", error),
    };
    let mut salt_bytes = [0u8; 32];
    secure_random_bytes(&mut salt_bytes);

    let mut derived_key = [0u8; 32];
    match hasher.hash_password_into(password, &salt_bytes, &mut derived_key) {
        Ok(_) => (),
        Err(error) => panic!("Error while deriving key: {:?}", error),
    }

    // We encrypt a 0x00 byte just so we have something to encrypt
    let (enc_result, nonce) = encrypt_data(&derived_key, SKELETON_ENCRYPTION_DATA.as_bytes());
    let cipher_text = match enc_result {
        Ok(cipher_text) => cipher_text,
        Err(error) => panic!("Error encrypting data: {:?}", error),
    };

    bytes.append(&mut DATABASE_WATERMARK.to_vec());
    bytes.append(&mut salt_bytes.to_vec());
    bytes.append(&mut nonce.to_vec());
    bytes.append(&mut cipher_text.to_vec());

    let write_result = file.write_all(&bytes[..]);

    match write_result {
        Ok(_) => (),
        Err(error) => panic!("Write failed: {:?}", error),
    }
}

pub fn read_database(path: &String, password: &[u8]) -> Result<String, String> {
    let file_result = OpenOptions::new().read(true).open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => return Err("File does not exist".to_string()),
            other_error => {
                panic!("Failed to open file: {:?}", other_error);
            }
        },
    };

    let mut file_buffer: Vec<u8> = vec![];

    match file.read_to_end(&mut file_buffer) {
        Ok(_) => (),
        Err(error) => return Err(format!("Failed to read database: {:?}", error)),
    }

    if file_buffer.len() < MIN_SIZE {
        return Err(format!("File too small! {}", file_buffer.len()));
    }

    if !(&file_buffer[0..8] == DATABASE_WATERMARK) {
        return Err("Not a RustKeep database".to_string());
    }

    let mut binding = ParamsBuilder::new();
    let hasher = match binding
        .m_cost(512 * 1024)
        .output_len(32)
        .p_cost(4)
        .t_cost(2)
        .context(Algorithm::Argon2id, Version::V0x13)
    {
        Ok(context) => context,
        Err(error) => return Err(format!("Failed to save params {:?}", error)),
    };

    // Salt, Nonce, Enc
    let salt = &file_buffer[8..40];
    let nonce = &file_buffer[40..52];
    let cipher_text = &file_buffer[52..];

    let mut derived_key = [0u8; 32];
    match hasher.hash_password_into(password, &salt, &mut derived_key) {
        Ok(_) => (),
        Err(error) => return Err(format!("Error while deriving key: {:?}", error)),
    }

    let decrypted_data = match decrypt_data(&derived_key, nonce, cipher_text) {
        Ok(text) => text,
        Err(error) => return Err(format!("Failed to decrypt data: {:?}", error)),
    };

    let as_utf8 = match str::from_utf8(decrypted_data.as_slice()) {
        Ok(text) => text,
        Err(error) => return Err(format!("Failed to decode UTF-8 data: {:?}", error)),
    };

    return Ok(as_utf8.to_string());
}

pub fn parse_database(raw: String) -> Result<Database, String> {
    let parsed: Database = match serde_json::from_str(raw.as_str()) {
        Ok(parsed_data) => parsed_data,
        Err(error) => return Err(format!("{:?}", error)),
    };

    return Ok(parsed);
}
