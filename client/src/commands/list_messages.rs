use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use chrono::DateTime;
use tabled::builder::Builder;
use tabled::settings::Style;

/// Lists all received messages.
/// 
/// Requires prior authentication.
pub fn list_messages() -> Result<()> {
    let username = utils::load_username()?;
    let (_, auth_key, _, _, _) = utils::load_keys()?;

    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::ListMessages {
        username: username.clone(),
        auth_key: auth_key,
    };

    network::write(&mut stream, message)?;

    let messages = match network::read(&mut stream)? {
        shared::frames::ServerFrame::ListMessagesResponse {
            message_previews: messages,
        } => messages,
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected answer from server")),
    };

    let mut builder = Builder::default();
    builder.push_record(vec!["File ID", "Sender", "File size", "Unlock time"]);
    for message in messages {
        let dt = DateTime::from_timestamp(message.unlock_timestamp as i64, 0);
        if dt.is_none() {
            continue;
        }
        let dt = dt.unwrap();
        builder.push_record(vec![
            message.message_id,
            message.sender_username,
            utils::format_file_size(message.file_size),
            dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        ]);
    }
    let mut table = builder.build();
    table.with(Style::modern_rounded());
    println!("{}", table);

    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;

    Ok(())
}
