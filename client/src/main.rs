mod cli;
mod commands;
mod crypto;
mod network;
mod utils;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::*;
use directories::ProjectDirs;
use std::fs;

fn main() -> Result<()> {
    // Initialize cryptography module
    shared::crypto::init()?;

    // Create the data directory for the app
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    fs::create_dir_all(dir.data_dir())?;

    // Parse command-line arguments using the defined CLI structure
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { user: username } => commands::login(username)?,
        Commands::Send {
            r#in: file,
            recipient: recipient_username,
            time: datetime,
        } => commands::send(file, recipient_username, datetime)?,
        Commands::List { list_command } => match list_command {
            ListCommands::Users => commands::list_users()?,
            ListCommands::Messages => commands::list_messages()?,
        },
        Commands::Signup { user: username } => commands::signup(username)?,
        Commands::Reset {} => commands::reset()?,
        Commands::Download { out: file, file_id } => commands::download(file, file_id)?,
        Commands::Unlock { r#in, file_id, out } => commands::unlock(r#in, out, file_id)?,
        Commands::Logout => commands::logout()?,
    };
    Ok(())
}
