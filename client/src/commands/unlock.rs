use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn unlock(filepath: &Path, message_id: &String) -> Result<()> {
    let username = utils::load_username()?;
    let (_, _, _, private_key, _, server_public_key) = utils::load_keys()?;

    let mut server_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut server_shared_key)?;

    let mut vec: Vec<u8> = Vec::new();
    vec.extend_from_slice(username.as_bytes());
    vec.extend_from_slice(message_id.as_bytes());

    let mut mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut mac, &server_shared_key, &vec);

    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::UnlockMessage {
        username: username.clone(),
        message_id: message_id.clone(),
        mac,
    };

    network::write(&mut stream, message)?;

    let (sender_public_key, encrypted_key, key_nonce, key_mac) = match network::read(&mut stream)? {
        shared::frames::ServerFrame::UnlockMessageResponse {
            sender_public_key,
            encrypted_key,
            key_nonce,
            key_mac,
        } => (sender_public_key, encrypted_key, key_nonce, key_mac),
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    let mut sender_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&sender_public_key, &private_key, &mut sender_shared_key)?;

    let mut key_vec: Vec<u8> = Vec::new();
    key_vec.extend_from_slice(&encrypted_key);
    key_vec.extend_from_slice(&key_nonce);
    if !verify_authentication(&key_mac, &sender_shared_key, &key_vec)? {
        return Err(anyhow!(
            "Verification failed, message may have been tampered with"
        ));
    }

    let mut decrypted_key: [u8; 32] = [0; KEY_SIZE];
    symmetric_decrypt(
        &key_nonce,
        &encrypted_key,
        &sender_shared_key,
        &mut decrypted_key,
    )?;

    let mut encrypted_data: Vec<u8> = Vec::new();
    let mut data_nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    let mut data_mac: [u8; MAC_SIZE] = [0; MAC_SIZE];

    let mut file = std::fs::File::open(filepath)?;
    file.read_exact(&mut data_mac)?;
    file.read_exact(&mut data_nonce)?;
    file.read_to_end(&mut encrypted_data)?;

    let mut decrypted_data: Vec<u8> = Vec::new();
    symmetric_decrypt(
        &data_nonce,
        &encrypted_data,
        &decrypted_key,
        &mut decrypted_data,
    )?;

    let mut file = std::fs::File::create(filepath)?;
    file.write_all(&decrypted_data)?;

    Ok(())
}
