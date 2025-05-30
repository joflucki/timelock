use crate::network;
use anyhow::{anyhow, Result};
use tabled::{builder::Builder, settings::Style};

/// Lists all existing users.
pub fn list_users() -> Result<()> {
    let mut stream = network::connect()?;
    let message = shared::frames::ClientFrame::ListUsers {};
    network::write(&mut stream, message)?;

    let usernames = match network::read(&mut stream)? {
        shared::frames::ServerFrame::ListUsersResponse { usernames } => usernames,
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    if usernames.is_empty() {
        println!("No users have signed up yet");
        return Ok(());
    }

    let mut builder = Builder::new();
    builder.push_record(vec!["Username"]);
    usernames
        .iter()
        .for_each(|username| builder.push_record(vec![username]));
    let mut table = builder.build();
    table.with(Style::modern_rounded());
    println!("{}", table);

    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;

    Ok(())
}
