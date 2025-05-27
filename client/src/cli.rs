use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "timelock")]
#[command(about = "Send and receive encrypted time capsules", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Login {
        #[arg(long)]
        username: String,
    },
    Logout,
    Signup {
        #[arg(long)]
        username: String,
    },
    Send {
        #[arg(long)]
        filepath: String,
        #[arg(long)]
        recipient_username: String,
        #[arg(long)]
        datetime: String,
    },
    List {
        #[command(subcommand)]
        list_command: ListCommands,
    },
    Download {
        #[arg(long)]
        filepath: String,
        file_id: String,
    },
    Unlock {
        #[arg(long)]
        filepath: String,
        #[arg(long)]
        file_id: String,
    },
    Reset {},
}

#[derive(Subcommand)]
pub enum ListCommands {
    Users,
    Messages,
}
