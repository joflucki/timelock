use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use std::net::TcpStream;

pub fn get_salt(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let path = dir.data_dir().join(username);
    let tree = sled::open(path)?;
    tree.get("key");
    Ok(())
}
