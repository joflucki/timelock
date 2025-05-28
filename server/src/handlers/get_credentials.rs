use crate::network;
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::crypto::{KEY_SIZE, NONCE_SIZE};
use std::net::TcpStream;
use subtle::ConstantTimeEq;

pub fn get_credentials(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let path = dir.data_dir().join(username).join("user_data");
    let db = sled::open(path)?;

    let stored_auth_key: [u8; KEY_SIZE] = db.get("auth_key")?.unwrap().as_ref().try_into()?;
    if !bool::from(stored_auth_key.ct_eq(auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    let encrypted_private_key: [u8; KEY_SIZE] = db
        .get("encrypted_private_key")?
        .unwrap()
        .as_ref()
        .try_into()?;
    let nonce: [u8; NONCE_SIZE] = db.get("nonce")?.unwrap().as_ref().try_into()?;

    network::write(
        stream,
        shared::frames::ServerFrame::GetCredentialsResponse {
            encrypted_private_key: encrypted_private_key,
            nonce: nonce,
        },
    )?;

    Ok(())
}
