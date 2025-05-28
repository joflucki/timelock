use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use tabled::settings::Style;
use tabled::Table;

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
        _ => return Err(anyhow!("Unexpected answer from server")),
    };

    let mut table = Table::new(messages);
    table.with(Style::modern_rounded());
    println!("{}", table);

    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;
    
    Ok(())
}
