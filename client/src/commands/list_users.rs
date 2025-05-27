use crate::network;
use anyhow::{anyhow, Result};

pub fn list_users() -> Result<Vec<String>> {
    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::ListUsers {};
    network::write(&mut stream, message)?;

    let usernames = match network::read(&mut stream)? {
        shared::frames::ServerFrame::ListUsersResponse { usernames } => usernames,
        _ => return Err(anyhow!("Unexpected response from server")),
    };
    Ok(usernames)
}
