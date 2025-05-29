use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use shared::crypto::KEY_SIZE;
use std::fs::{self, File};
use std::io::{Read, Write};

/// Loads cryptographic keys from the default file.
///
/// The keys are expected to be in a specific order:
/// master_key, auth_key, enc_key, private_key, public_key
pub fn load_keys() -> Result<(
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
)> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let mut file = File::open(dir.data_dir().join("credentials"))?;

    let mut master_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut enc_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut public_key: [u8; KEY_SIZE] = [0; KEY_SIZE];

    file.read_exact(&mut master_key)?;
    file.read_exact(&mut auth_key)?;
    file.read_exact(&mut enc_key)?;
    file.read_exact(&mut private_key)?;
    file.read_exact(&mut public_key)?;

    // Read other keys here in the same order
    Ok((master_key, auth_key, enc_key, private_key, public_key))
}

/// Saves cryptographic keys to the default file.
pub fn save_keys(
    master_key: &[u8; KEY_SIZE],
    auth_key: &[u8; KEY_SIZE],
    enc_key: &[u8; KEY_SIZE],
    private_key: &[u8; KEY_SIZE],
    public_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    fs::create_dir_all(dir.data_dir())?;
    let mut file = File::create(dir.data_dir().join("credentials"))?;

    file.write_all(master_key)?;
    file.write_all(auth_key)?;
    file.write_all(enc_key)?;
    file.write_all(private_key)?;
    file.write_all(public_key)?;

    Ok(())
}

pub fn delete_keys() -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let path = dir.data_dir().join("credentials");
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn save_username(username: &str) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    fs::create_dir_all(dir.data_dir())?;
    let mut file = File::create(dir.data_dir().join("username"))?;
    file.write_all(username.as_bytes())?;
    Ok(())
}

pub fn load_username() -> Result<String> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let mut file = File::open(dir.data_dir().join("username"))?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username.trim().to_string())
}
pub fn delete_username() -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Client") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let path = dir.data_dir().join("username");
    std::fs::remove_file(path)?;
    Ok(())
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}