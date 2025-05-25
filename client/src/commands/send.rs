use crate::crypto::*;
use crate::network;
use crate::utils;
use chrono::DateTime;
use chrono::FixedOffset;
use shared::crypto::*;
use shared::messages::ClientMessage;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn send(filepath: &Path, recipient_username: &String, datetime: &DateTime<FixedOffset>) {
    // Connect to the server
    let mut stream: native_tls::TlsStream<std::net::TcpStream> =
        network::connect().expect("Failed to connect to server");

    // Fetch recipient's public key
    network::write(
        &mut stream,
        shared::messages::ClientMessage::GetPublicKey {
            id: recipient_username.clone(),
        },
    )
    .expect("Error sending public key request to server");

    let recipient_public_key: [u8; KEY_SIZE] =
        match network::read(&mut stream).expect("Error reading response from server") {
            shared::messages::ServerMessage::GetPublicKeyResponse { public_key } => public_key,
            _ => panic!("Failed to get recipient's public key"),
        };

    // Load credentials
    let username = utils::load_username().expect("Error loading username");
    let (_, _, _, private_key, _, server_public_key) =
        utils::load_keys().expect("Error loading keys");

    let mut recipient_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(
        &recipient_public_key,
        &private_key,
        &mut recipient_shared_key,
    );

    // Initialize nonces
    let mut key_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    let mut data_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];

    random_buffer(&mut key_nonce);
    random_buffer(&mut data_nonce);

    // Initialize one-time key
    let mut one_time_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    random_buffer(&mut one_time_key);

    // Encrypt the one-time key
    let mut encrypted_one_time_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_encrypt(
        &key_nonce,
        &one_time_key,
        &recipient_shared_key,
        &mut encrypted_one_time_key,
    );

    // Authenticate the one-time key and its nonce
    let mut key_vec: Vec<u8> = Vec::new();
    key_vec.extend_from_slice(&encrypted_one_time_key);
    key_vec.extend_from_slice(&key_nonce);

    let mut key_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut key_mac, &recipient_shared_key, &key_vec);

    // Encrypt the file data
    let mut data: Vec<u8> = Vec::new();
    File::open(filepath)
        .expect("Failed to open file")
        .read(&mut data)
        .expect("Failed to read file");

    let mut encrypted_data: Vec<u8> = Vec::new();
    symmetric_encrypt(&data_nonce, &data, &one_time_key, &mut encrypted_data);

    // Authenticate the encrypted message and its nonce
    let mut data_vec: Vec<u8> = Vec::new();
    data_vec.extend_from_slice(&encrypted_data);
    data_vec.extend_from_slice(&data_nonce);

    let mut data_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut data_mac, &recipient_shared_key, &data_vec);

    // Authenticate the full message
    let mut server_shared_key: [u8; 32] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut server_shared_key);

    let mut full_message_vec: Vec<u8> = Vec::new();

    full_message_vec.extend_from_slice(username.as_bytes());
    full_message_vec.extend_from_slice(recipient_username.as_bytes());
    full_message_vec.extend_from_slice(&datetime.timestamp().to_be_bytes());
    full_message_vec.extend_from_slice(&encrypted_one_time_key);
    full_message_vec.extend_from_slice(&key_nonce);
    full_message_vec.extend_from_slice(&key_mac);
    full_message_vec.extend_from_slice(&encrypted_data);
    full_message_vec.extend_from_slice(&data_nonce);
    full_message_vec.extend_from_slice(&data_mac);

    let mut final_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut final_mac, &server_shared_key, &full_message_vec);

    network::write(
        &mut stream,
        ClientMessage::SendMessage {
            sender_username: username.clone(),
            recipient_username: recipient_username.clone(),
            timestamp: datetime.timestamp().to_be_bytes(),
            encrypted_key: encrypted_one_time_key,
            key_nonce,
            key_mac,
            encrypted_data,
            data_nonce,
            data_mac,
            mac: final_mac,
        },
    )
    .expect("Error sending message to server");

    if match network::read(&mut stream).expect("Error reading response from server") {
        shared::messages::ServerMessage::SendMessageResponse { ok } => ok,
        _ => panic!("Unexpected response from server"),
    } {
        println!("Message sent successfully!");
    } else {
        panic!("Failed to send message");
    }
}
