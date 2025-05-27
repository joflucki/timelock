use crate::crypto::*;
use crate::network;
use crate::utils;
use shared::crypto::*;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn unlock(filepath: &Path, message_id: &String) {
    let username = utils::load_username().expect("Failed to load username");
    let (_, _, _, private_key, _, server_public_key) =
        utils::load_keys().expect("Failed to load keys");

    let mut server_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut server_shared_key);

    let mut vec: Vec<u8> = Vec::new();
    vec.extend_from_slice(username.as_bytes());
    vec.extend_from_slice(message_id.as_bytes());

    let mut mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut mac, &server_shared_key, &vec);

    let mut stream = network::connect().expect("Failed to connect to server");
    let message = shared::messages::ClientMessage::UnlockMessage {
        username: username.clone(),
        message_id: message_id.clone(),
        mac,
    };

    network::write(&mut stream, message).expect("Failed to send request");

    let (sender_public_key, encrypted_key, key_nonce, key_mac) =
        match network::read(&mut stream).expect("Failed to read response from server") {
            shared::messages::ServerMessage::UnlockMessageResponse {
                sender_public_key,
                encrypted_key,
                key_nonce,
                key_mac,
            } => Some((sender_public_key, encrypted_key, key_nonce, key_mac)),
            _ => None,
        }
        .expect("Failed to parse response from server");

    let mut sender_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&sender_public_key, &private_key, &mut sender_shared_key);

    let mut key_vec: Vec<u8> = Vec::new();
    key_vec.extend_from_slice(&encrypted_key);
    key_vec.extend_from_slice(&key_nonce);
    let ok = verify_authentication(&key_mac, &sender_shared_key, &key_vec);
    if !ok {
        panic!("Verification failed, message may have been tampered with");
    }

    let mut decrypted_key: [u8; 32] = [0; KEY_SIZE];
    symmetric_decrypt(
        &key_nonce,
        &encrypted_key,
        &sender_shared_key,
        &mut decrypted_key,
    );

    let mut encrypted_data: Vec<u8> = Vec::new();
    let mut data_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    let mut data_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];

    let mut file = std::fs::File::open(filepath).expect("Failed to open file");
    file.read_exact(&mut data_mac)
        .expect("Failed to read MAC from file");
    file.read_exact(&mut data_nonce)
        .expect("Failed to read nonce from file");
    file.read_to_end(&mut encrypted_data)
        .expect("Failed to read data from file");

    let mut decrypted_data: Vec<u8> = Vec::new();
    symmetric_decrypt(
        &data_nonce,
        &encrypted_data,
        &decrypted_key,
        &mut decrypted_data,
    );

    let mut file = std::fs::File::create(filepath).expect("Failed to open file");
    file.write_all(&decrypted_data)
        .expect("Failed to write data to file");
}
