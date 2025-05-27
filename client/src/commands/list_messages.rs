use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use shared::models::MessagePreview;

pub fn list_messages() -> Result<Vec<MessagePreview>> {
    let username = utils::load_username()?;
    let (_, _, _, private_key, _, server_public_key) = utils::load_keys()?;

    let mut shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut shared_key)?;

    let mut mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut mac, &shared_key, username.as_bytes());

    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::ListMessages {
        username: username.clone(),
        mac,
    };

    network::write(&mut stream, message)?;

    let messages = match network::read(&mut stream)? {
        shared::frames::ServerFrame::ListMessagesResponse {
            message_previews: messages,
        } => messages,
        _ => return Err(anyhow!("Unexpected answer from server")),
    };

    Ok(messages)
}
