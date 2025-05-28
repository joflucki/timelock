use crate::network;
use anyhow::{anyhow, Result};
use tabled::{settings::Style, Table};

pub fn list_users() -> Result<()> {
    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::ListUsers {};
    network::write(&mut stream, message)?;

    let usernames = match network::read(&mut stream)? {
        shared::frames::ServerFrame::ListUsersResponse { usernames } => usernames,
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    let mut table = Table::new(usernames);
    table.with(Style::modern_rounded());
    println!("{}", table);
    Ok(())
}
