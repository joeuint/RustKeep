use rand::{Rng, RngCore, SeedableRng};
use rand_chacha;

pub fn secure_random_bytes(slice: &mut [u8]) {
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    let _ = &rng.fill_bytes(slice);
}

pub fn random_number_in_range(low: i32, high: i32) -> i32 {
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();

    let random: i32 = rng.gen_range(low..high);

    return random;
}

pub fn random_chars(amount: u16, char_set: &[u8]) -> String {
    let mut rng = rand_chacha::ChaCha20Rng::from_entropy();
    let result: String = (0..amount as usize)
        .map(|_| {
            let idx = rng.gen_range(0..char_set.len());
            char_set[idx] as char
        })
        .collect();

    return result;
}
