use crate::crypto::*;
use crate::network;
use crate::utils;
use shared::crypto::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn download(filepath: &Path, message_id: &String) {
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
    let message = shared::messages::ClientMessage::DownloadMessage {
        username: username.clone(),
        message_id: message_id.clone(),
        mac,
    };

    network::write(&mut stream, message).expect("Failed to send request");

    let (sender_public_key, encrypted_data, data_nonce, data_mac) =
        match network::read(&mut stream).expect("Failed to read response from server") {
            shared::messages::ServerMessage::DownloadMessageResponse {
                sender_public_key,
                encrypted_data,
                data_nonce,
                data_mac,
            } => Some((sender_public_key, encrypted_data, data_nonce, data_mac)),
            _ => None,
        }
        .expect("Failed to parse response from server");

    let mut sender_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&sender_public_key, &private_key, &mut sender_shared_key);

    let mut data_vec: Vec<u8> = Vec::new();
    data_vec.extend_from_slice(&encrypted_data);
    data_vec.extend_from_slice(&data_nonce);

    let ok = verify_authentication(&data_mac, &sender_shared_key, &data_vec);
    if !ok {
        panic!("Verification failed, message may have been tampered with");
    }

    File::create(filepath)
        .expect("Failed to create file")
        .write_all(&encrypted_data)
        .expect("Failed to write data to file");

    println!("Message downloaded successfully to {}", filepath.display());
}
