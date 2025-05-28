use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::crypto::SALT_SIZE;
use std::net::TcpStream;

use crate::network;

pub fn get_salt(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
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
    let salt: [u8; SALT_SIZE] = db.get("salt")?.unwrap().as_ref().try_into()?;

    network::write(
        stream,
        shared::frames::ServerFrame::GetSaltResponse { salt: salt },
    )?;

    Ok(())
}
