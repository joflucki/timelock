mod cli;
mod commands;
mod crypto;
mod network;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::*;
use std::path::Path;
use utils::{display_messages, display_users, parse_datetime};

fn main() -> Result<()> {
    shared::crypto::init()?;

    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { username } => commands::login(username)?,
        Commands::Send {
            filepath,
            recipient_username,
            datetime,
        } => commands::send(
            Path::new(filepath),
            recipient_username,
            &parse_datetime(datetime)?,
        )?,
        Commands::List { list_command } => match list_command {
            ListCommands::Users => display_users(commands::list_users()?),
            ListCommands::Messages => display_messages(commands::list_messages()?),
        },
        Commands::Signup { username } => commands::signup(username)?,
        Commands::Reset {} => commands::reset()?,
        Commands::Download { filepath, file_id } => {
            commands::download(Path::new(filepath), file_id)?
        }
        Commands::Unlock { filepath, file_id } => commands::unlock(Path::new(filepath), file_id)?,
        Commands::Logout => commands::logout()?,
    };

    Ok(())
}
