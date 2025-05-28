mod cli;
mod commands;
mod crypto;
mod network;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::*;

fn main() -> Result<()> {
    shared::crypto::init()?;

    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { username } => commands::login(username)?,
        Commands::Send {
            filepath,
            recipient_username,
            datetime,
        } => commands::send(filepath, recipient_username, datetime)?,
        Commands::List { list_command } => match list_command {
            ListCommands::Users => commands::list_users()?,
            ListCommands::Messages => commands::list_messages()?,
        },
        Commands::Signup { username } => commands::signup(username)?,
        Commands::Reset {} => commands::reset()?,
        Commands::Download { filepath, file_id } => commands::download(filepath, file_id)?,
        Commands::Unlock { filepath, file_id } => commands::unlock(filepath, file_id)?,
        Commands::Logout => commands::logout()?,
    };
    Ok(())
}
