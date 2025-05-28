use crate::{network, utils};
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::{
    crypto::*,
    models::{MessageDataFile, MessageMetadataFile},
};
use std::{
    fs::{self, File},
    io::Write,
    net::TcpStream,
};
use subtle::ConstantTimeEq;
use uuid::Uuid;

pub fn send_message(
    stream: &mut TlsStream<TcpStream>,
    sender_username: &str,
    recipient_username: &str,
    timestamp: u64,
    encrypted_key: &[u8; KEY_SIZE],
    key_nonce: &[u8; NONCE_SIZE],
    key_mac: &[u8; MAC_SIZE],
    encrypted_data: Vec<u8>,
    data_nonce: &[u8; NONCE_SIZE],
    data_mac: &[u8; MAC_SIZE],
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let (stored_auth_key, _, _, _, _) = utils::load_credentials(sender_username)?;
    if !bool::from(stored_auth_key.ct_eq(auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    let metadata = MessageMetadataFile {
        sender_username: sender_username.to_string(),
        unlock_timestamp: timestamp,
        encrypted_key: encrypted_key.clone(),
        key_nonce: key_nonce.clone(),
        key_mac: key_mac.clone(),
    };

    let data = MessageDataFile {
        encrypted_data,
        data_nonce: data_nonce.clone(),
        data_mac: data_mac.clone(),
    };

    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };

    let file_list: Vec<String> = fs::read_dir(dir.data_dir())?
        .map(|result| result.expect("Could not list file"))
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .collect();

    if !file_list.contains(&recipient_username.to_string()) {
        return Err(anyhow!("Recipient does not exist"));
    }

    let uuid = Uuid::new_v4().to_string();
    let path = dir
        .data_dir()
        .join(recipient_username)
        .join("messages")
        .join(uuid.clone());
    fs::create_dir_all(path)?;
    print!("Created message folder");

    let path = dir
        .data_dir()
        .join(recipient_username)
        .join("messages")
        .join(uuid.clone())
        .join("metadata");
    let mut file = File::create(path)?;
    file.write_all(&bincode::serialize(&metadata)?)?;
    print!("Wrote metadata");

    let path = dir
        .data_dir()
        .join(recipient_username)
        .join("messages")
        .join(uuid.clone())
        .join("data");
    let mut file = File::create(path)?;
    file.write_all(&bincode::serialize(&data)?)?;
    print!("Wrote data");

    network::write(stream, shared::frames::ServerFrame::SendMessageResponse {})?;

    Ok(())
}
