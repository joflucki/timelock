use std::{
    fs::{self, File},
    io::{Read, Write},
};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use shared::crypto::{KEY_SIZE, NONCE_SIZE, SALT_SIZE};

/// Returns (`auth_key`, `encrypted_private_key`, `public_key`, `nonce`, `salt`)
pub fn load_credentials(
    username: &str,
) -> Result<(
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
    [u8; KEY_SIZE],
    [u8; NONCE_SIZE],
    [u8; SALT_SIZE],
)> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    let mut file = File::open(dir.data_dir().join(username).join("credentials"))?;

    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut encrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut public_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    let mut salt: [u8; SALT_SIZE] = [0; SALT_SIZE];

    // Add other key buffers here
    file.read_exact(&mut auth_key)?;
    file.read_exact(&mut encrypted_private_key)?;
    file.read_exact(&mut public_key)?;
    file.read_exact(&mut nonce)?;
    file.read_exact(&mut salt)?;

    // Read other keys here in the same order
    Ok((auth_key, encrypted_private_key, public_key, nonce, salt))
}

pub fn save_credentials(
    username: &str,
    auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    public_key: &[u8; KEY_SIZE],
    nonce: &[u8; NONCE_SIZE],
    salt: &[u8; SALT_SIZE],
) -> Result<()> {
    let dir = match ProjectDirs::from("ch", "Timelock", "Timelock Server") {
        Some(dir) => dir,
        None => {
            return Err(anyhow!(
                "No valid home directory path could be retrieved from the operating system"
            ))
        }
    };
    fs::create_dir_all(dir.data_dir().join(username))?;
    let mut file = File::create(dir.data_dir().join(username).join("credentials"))?;

    file.write_all(auth_key)?;
    file.write_all(encrypted_private_key)?;
    file.write_all(public_key)?;
    file.write_all(nonce)?;
    file.write_all(salt)?;

    Ok(())
}