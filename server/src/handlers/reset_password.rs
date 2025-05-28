use crate::network;
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;
use subtle::ConstantTimeEq;

pub fn reset_password(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    new_auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    salt: &[u8; SALT_SIZE],
    nonce: &[u8; NONCE_SIZE],
    old_auth_key: &[u8; KEY_SIZE],
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
    if !bool::from(stored_auth_key.ct_eq(old_auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    db.insert("auth_key", new_auth_key)?;
    db.insert("encrypted_private_key", encrypted_private_key)?;
    db.insert("salt", salt)?;
    db.insert("nonce", nonce)?;

    network::write(
        stream,
        shared::frames::ServerFrame::ResetPasswordResponse {},
    )?;

    Ok(())
}
