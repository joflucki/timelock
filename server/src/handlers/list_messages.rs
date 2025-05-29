use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use native_tls::TlsStream;
use shared::{
    crypto::*,
    models::{MessageMetadataFile, MessagePreview},
};
use std::{
    fs::{self, File},
    io::Read,
    net::TcpStream,
};
use subtle::ConstantTimeEq;

use crate::{network, utils};

pub fn list_messages(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let (stored_auth_key, _, _, _, _) = utils::load_credentials(username)?;
    if !bool::from(stored_auth_key.ct_eq(auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    let path = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let mut previews: Vec<MessagePreview> = Vec::new();
    let path = path.data_dir().join(username).join("messages");
    for result in fs::read_dir(path)? {
        if let Err(e) = result {
            return Err(anyhow!(e));
        }
        let entry = result.unwrap();
        let mut metadata_file = File::open(entry.path().join("metadata"))?;
        let data_file = File::open(entry.path().join("data"))?;
        let mut bytes: Vec<u8> = Vec::new();
        metadata_file.read_to_end(&mut bytes)?;
        let metadata: MessageMetadataFile = bincode::deserialize(&bytes)?;
        previews.push(MessagePreview {
            message_id: entry
                .file_name()
                .into_string()
                .map_err(|_| anyhow!("Could not read file name"))?,
            sender_username: metadata.sender_username.clone(),
            unlock_timestamp: metadata.unlock_timestamp,
            file_size: data_file.metadata()?.len(),
        });
    }

    network::write(
        stream,
        shared::frames::ServerFrame::ListMessagesResponse {
            message_previews: previews,
        },
    )?;

    Ok(())
}
