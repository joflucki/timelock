use clap::{Parser, Subcommand};

/// The main CLI for interacting with the timelock service.
#[derive(Parser)]
#[command(name = "timelock")]
#[command(about = "Send and receive encrypted time capsules")]
#[command(
    long_about = "The Timelock CLI allows users to send and receive encrypted messages that can be unlocked only at a specified future time."
)]
pub struct Cli {
    #[command(subcommand)]
    /// Available subcommands for the `timelock` CLI.
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Logs in using an existing account.
    Login {
        /// The username to log in with.
        #[arg(long, help = "Username to log in with.")]
        user: String,
    },

    /// Logs out of the timelock service.
    Logout,

    /// Signs up for a new account.
    Signup {
        /// The username to sign up with.
        #[arg(long, help = "Username to create a new account.")]
        user: String,
    },

    /// Sends an encrypted time capsule to a recipient.
    Send {
        /// The file to send as a time capsule.
        #[arg(long, help = "The file to send.")]
        r#in: String,

        /// The recipient of the time capsule.
        #[arg(long, help = "Recipient username of the time capsule.")]
        recipient: String,

        /// The time when the capsule can be unlocked.
        #[arg(
            long,
            help = "The UTC datetime indicating when the capsule can be unlocked, in format 'YYYY-MM-DD HH:MM:SS'"
        )]
        time: String,
    },

    /// Lists various items.
    List {
        #[command(subcommand)]
        /// Available list-related commands.
        list_command: ListCommands,
    },

    /// Downloads a file using a provided file ID.
    Download {
        /// The file to download.
        #[arg(long, help = "Where to output the file.")]
        out: String,

        /// The file ID to fetch the corresponding file.
        #[arg(long, help = "The unique file ID.")]
        file_id: String,
    },

    /// Unlocks a previously downloaded time capsule.
    Unlock {
        /// The file to unlock.
        #[arg(long, help = "The file to unlock.")]
        r#in: String,

        /// The location of the output file.
        #[arg(long, help = "Where to output the unlocked file.")]
        out: String,

        /// The file ID of the time capsule to unlock.
        #[arg(long, help = "The unique file ID to unlock.")]
        file_id: String,
    },

    /// Resets the user password.
    Reset {},
}

#[derive(Subcommand)]
pub enum ListCommands {
    /// Lists all users in the system.
    Users,

    /// Lists all received messages.
    Messages,
}
