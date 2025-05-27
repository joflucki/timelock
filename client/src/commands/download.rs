use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn download(filepath: &Path, message_id: &String) -> Result<()> {
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
    let message = shared::frames::ClientFrame::DownloadMessage {
        username: username.clone(),
        message_id: message_id.clone(),
        mac,
    };

    network::write(&mut stream, message)?;

    let (sender_public_key, encrypted_data, data_nonce, data_mac) =
        match network::read(&mut stream)? {
            shared::frames::ServerFrame::DownloadMessageResponse {
                sender_public_key,
                encrypted_data,
                data_nonce,
                data_mac,
            } => (sender_public_key, encrypted_data, data_nonce, data_mac),
            _ => return Err(anyhow!("Unexpected answer from server")),
        };

    let mut sender_shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&sender_public_key, &private_key, &mut sender_shared_key)?;

    let mut data_vec: Vec<u8> = Vec::new();
    data_vec.extend_from_slice(&encrypted_data);
    data_vec.extend_from_slice(&data_nonce);

    if !verify_authentication(&data_mac, &sender_shared_key, &data_vec)? {
        return Err(anyhow!(
            "Authentication tag verification has failed, message may have been tampered with"
        ));
    }

    let mut file = File::create(filepath)?;
    file.write_all(&data_mac)?;
    file.write_all(&data_nonce)?;
    file.write_all(&encrypted_data)?;

    Ok(())
}
