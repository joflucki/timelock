mod cli;
mod commands;
mod crypto;
mod network;
mod utils;

use chrono::NaiveDateTime;
use clap::Parser;
use cli::*;
use std::path::Path;

fn main() {
    // Initialize the cryptography module.
    // If it fails, nothing works
    // and we should panic
    let ret = shared::crypto::init();
    if ret != 0 {
        panic!("Cryptography module initialization failed")
    }

    // Parse command line arguments
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { username } => commands::login(username),
        Commands::Send {
            filepath,
            recipient_username,
            timestamp,
        } => commands::send(
            Path::new(filepath),
            recipient_username,
            &NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S")
                .expect("Invalid date-time format")
                .and_utc(),
        ),
        Commands::List { list_command } => match list_command {
            ListCommands::Users => commands::list_users(),
            ListCommands::Messages => commands::list_messages(),
        },
        Commands::Signup { username } => commands::signup(username),
        Commands::Reset {} => commands::reset(),
        Commands::Download { filepath, file_id } => {
            commands::download(Path::new(filepath), file_id)
        }
        Commands::Unlock { filepath, file_id } => commands::unlock(Path::new(filepath), file_id),
        Commands::Logout => commands::logout(),
    }
}
