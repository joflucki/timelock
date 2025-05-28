use crate::{network, utils};
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::{crypto::*, frames::ServerFrame};
use std::{fs, net::TcpStream};

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

    utils::save_credentials(
        username,
        auth_key,
        encrypted_private_key,
        public_key,
        nonce,
        salt,
    )?;

    fs::create_dir_all(path.join("messages"))?;

    let frame = ServerFrame::IdentifyResponse {};
    network::write(stream, frame)?;

    Ok(())
}
