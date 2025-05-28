use crate::network;
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::{crypto::*, frames::ServerFrame};
use std::net::TcpStream;

pub fn identify(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    public_key: &[u8; KEY_SIZE],
    auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    salt: &[u8; SALT_SIZE],
    nonce: &[u8; NONCE_SIZE],
) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let path = dir.data_dir().join(username);
    if path.try_exists()? {
        return Err(anyhow!("Username already in use"));
    }
    let path = path.join("user_data");
    let db = sled::open(path)?;
    db.insert("public_key", public_key)?;
    db.insert("auth_key", auth_key)?;
    db.insert("encrypted_private_key", encrypted_private_key)?;
    db.insert("salt", salt)?;
    db.insert("nonce", nonce)?;

    let frame = ServerFrame::IdentifyResponse {};
    network::write(stream, frame)?;

    Ok(())
}
