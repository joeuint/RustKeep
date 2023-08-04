use super::random::{random_chars, random_number_in_range};
use std::vec;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                abcdefghijklmnopqrstuvwxyz\
                0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

pub fn generate_passphrase(amount: u16, read_string: String) -> Vec<String> {
    let mut words: Vec<String> = vec![];

    for _ in 0..amount {
        let split_string = read_string.as_str().split("\n");
        let mut count = 0;
        let word_vec: Vec<&str> = split_string.collect();

        for _ in &word_vec {
            count = count + 1
        }

        words.append(&mut vec![word_vec
            [random_number_in_range(0, count - 1) as usize]
            .to_string()])
    }

    return words;
}

pub fn generate_password(length: u16) -> String {
    return random_chars(length, CHARSET);
}
