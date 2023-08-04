use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Error, Key, Nonce,
};

pub fn encrypt_data(key: &[u8], data: &[u8]) -> (Result<Vec<u8>, Error>, Vec<u8>) {
    let converted_key = Key::<Aes256Gcm>::from_slice(key);

    let cipher = Aes256Gcm::new(converted_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypt_result = cipher.encrypt(&nonce, data);

    // The Nonce is returned as a vec because yes
    return (encrypt_result, nonce.to_vec());
}

pub fn decrypt_data(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>, Error> {
    let converted_key = Key::<Aes256Gcm>::from_slice(key);

    let cipher = Aes256Gcm::new(converted_key);

    return cipher.decrypt(Nonce::from_slice(nonce), data);
}
