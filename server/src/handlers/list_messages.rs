use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;

pub fn list_messages(
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
    todo!()
}
