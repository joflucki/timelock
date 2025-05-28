use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use std::{fs, net::TcpStream};

use crate::network;

pub fn list_users(stream: &mut TlsStream<TcpStream>) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let file_list: Vec<String> = fs::read_dir(dir.data_dir())?
        .map(|result| result.expect(""))
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .collect();

    network::write(
        stream,
        shared::frames::ServerFrame::ListUsersResponse {
            usernames: file_list,
        },
    )?;

    Ok(())
}
