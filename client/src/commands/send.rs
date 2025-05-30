use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use chrono::Utc;
use shared::crypto::*;
use shared::frames::ClientFrame;
use std::fs::File;
use std::io::Read;

/// Sends a time capsule to the specified user.
///
/// Requires prior authentication.
pub fn send(filepath: &String, recipient_username: &String, datetime: &String) -> Result<()> {
    let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S")?.and_utc();
    if datetime.timestamp() < Utc::now().timestamp() {
        return Err(anyhow!("A message can not unlock in the past."));
    }

    // Connect to the server
    let mut stream: native_tls::TlsStream<std::net::TcpStream> = network::connect()?;

    // Fetch recipient's public key
    network::write(
        &mut stream,
        shared::frames::ClientFrame::GetPublicKey {
            username: recipient_username.clone(),
        },
    )?;

    let recipient_public_key: [u8; KEY_SIZE] = match network::read(&mut stream)? {
        shared::frames::ServerFrame::GetPublicKeyResponse { public_key } => public_key,
        _ => return Err(anyhow!("Unexpected answer from server")),
    };

    // Load credentials
    let username = utils::load_username()?;
    let (_, auth_key, _, private_key, _) = utils::load_keys()?;

    let mut recipient_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(
        &recipient_public_key,
        &private_key,
        &mut recipient_shared_key,
    )?;

    // Initialize nonces
    let mut key_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    let mut data_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];

    random_buffer(&mut key_nonce)?;
    random_buffer(&mut data_nonce)?;

    // Initialize one-time key
    let mut one_time_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    random_buffer(&mut one_time_key)?;

    // Encrypt the one-time key
    let mut encrypted_one_time_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_encrypt(
        &key_nonce,
        &one_time_key,
        &recipient_shared_key,
        &mut encrypted_one_time_key,
    )?;

    // Authenticate the one-time key and its nonce
    let mut key_vec: Vec<u8> = Vec::new();
    key_vec.extend_from_slice(&encrypted_one_time_key);
    key_vec.extend_from_slice(&key_nonce);

    let mut key_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut key_mac, &recipient_shared_key, &key_vec);

    // Encrypt the file data
    let mut data: Vec<u8> = Vec::new();
    File::open(filepath)?.read_to_end(&mut data)?;
    let mut encrypted_data: Vec<u8> = vec![0; data.len()]; // Must pre-allocate for libsodium unsafe code
    symmetric_encrypt(&data_nonce, &data, &one_time_key, &mut encrypted_data)?;

    // Authenticate the encrypted message and its nonce
    let mut data_vec: Vec<u8> = Vec::new();
    data_vec.extend_from_slice(&encrypted_data);
    data_vec.extend_from_slice(&data_nonce);

    let mut data_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut data_mac, &recipient_shared_key, &data_vec);

    network::write(
        &mut stream,
        ClientFrame::SendMessage {
            sender_username: username.clone(),
            recipient_username: recipient_username.clone(),
            timestamp: datetime.timestamp() as u64,
            encrypted_key: encrypted_one_time_key,
            key_nonce,
            key_mac,
            encrypted_data,
            data_nonce,
            data_mac,
            auth_key: auth_key,
        },
    )?;
    match network::read(&mut stream)? {
        shared::frames::ServerFrame::SendMessageResponse {} => {}
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected answer from server")),
    }
    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;
    println!("Message sent successfully to {}", recipient_username);
    Ok(())
}
