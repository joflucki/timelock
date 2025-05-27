use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use directories::ProjectDirs;
use shared::models::MessagePreview;
use std::fs::File;
use std::io::{Read, Write};
use tabled::Table;

/// Loads cryptographic keys from the default file.
///
/// The keys are expected to be in a specific order:
/// master_key, auth_key, enc_key, private_key, public_key, server_public_key
pub fn load_keys() -> Result<([u8; 32], [u8; 32], [u8; 32], [u8; 32], [u8; 32], [u8; 32])> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let mut file = File::open(dir.data_dir().join("credentials"))?;

    let mut master_key: [u8; 32] = [0; 32];
    let mut auth_key: [u8; 32] = [0; 32];
    let mut enc_key: [u8; 32] = [0; 32];
    let mut private_key: [u8; 32] = [0; 32];
    let mut public_key: [u8; 32] = [0; 32];
    let mut server_public_key: [u8; 32] = [0; 32];

    // Add other key buffers here
    file.read_exact(&mut master_key)?;
    file.read_exact(&mut auth_key)?;
    file.read_exact(&mut enc_key)?;
    file.read_exact(&mut private_key)?;
    file.read_exact(&mut public_key)?;
    file.read_exact(&mut server_public_key)?;

    // Read other keys here in the same order
    Ok((
        master_key,
        auth_key,
        enc_key,
        private_key,
        public_key,
        server_public_key,
    ))
}

/// Saves cryptographic keys to the default file.
pub fn save_keys(
    master_key: &[u8; 32],
    auth_key: &[u8; 32],
    enc_key: &[u8; 32],
    private_key: &[u8; 32],
    public_key: &[u8; 32],
    server_public_key: &[u8; 32],
) -> Result<()> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let mut file = File::create(dir.data_dir().join("credentials"))?;

    file.write_all(master_key)?;
    file.write_all(auth_key)?;
    file.write_all(enc_key)?;
    file.write_all(private_key)?;
    file.write_all(public_key)?;
    file.write_all(server_public_key)?;

    // Add other keys here if you have more
    Ok(())
}

pub fn delete_keys() -> Result<()> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let path = dir.data_dir().join("credentials");
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn save_username(username: &str) -> Result<()> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let mut file = File::create(dir.data_dir().join("username"))?;
    file.write_all(username.as_bytes())?;
    Ok(())
}

pub fn load_username() -> Result<String> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let mut file = File::open(dir.data_dir().join("username"))?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}
pub fn delete_username() -> Result<()> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let path = dir.data_dir().join("username");
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn parse_datetime(str: &str) -> Result<DateTime<Utc>> {
    Ok(NaiveDateTime::parse_from_str(str, "%Y-%m-%d %H:%M:%S")?.and_utc())
}
pub fn display_users(users: Vec<String>) {
    let table = Table::new(users);
    println!("{}", table)
}
pub fn display_messages(messages: Vec<MessagePreview>) {
    let table = Table::new(messages);
    println!("{}", table)
}
