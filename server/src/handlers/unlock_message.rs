use anyhow::{anyhow, Result};
use chrono::Utc;
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::{crypto::KEY_SIZE, models::MessageMetadataFile};
use std::{fs::File, io::Read, net::TcpStream};
use subtle::ConstantTimeEq;

use crate::{network, utils};

pub fn unlock_message(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    message_id: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let (stored_auth_key, _, _, _, _) = utils::load_credentials(username)?;
    if !bool::from(stored_auth_key.ct_eq(auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };

    let path = dir
        .data_dir()
        .join(username)
        .join("messages")
        .join(message_id)
        .join("metadata");
    let mut file = File::open(path)?;
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    let metadata: MessageMetadataFile = bincode::deserialize(&bytes)?;

    if (Utc::now().timestamp() as u64) < metadata.unlock_timestamp {
        return Err(anyhow!("Message can not be unlocked at this time"));
    };

    let (_, _, sender_public_key, _, _) = utils::load_credentials(&metadata.sender_username)?;

    network::write(
        stream,
        shared::frames::ServerFrame::UnlockMessageResponse {
            sender_public_key: sender_public_key,
            encrypted_key: metadata.encrypted_key,
            key_nonce: metadata.key_nonce,
            key_mac: metadata.key_mac,
        },
    )?;

    Ok(())
}
