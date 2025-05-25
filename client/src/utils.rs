use directories::ProjectDirs;
use std::fs::File;
use std::io::{Read, Result, Write};

/// Loads cryptographic keys from the default file.
///
/// The keys are expected to be in a specific order:
/// master_key, auth_key, enc_key, private_key, public_key, server_public_key
pub fn load_keys() -> Result<([u8; 32], [u8; 32], [u8; 32], [u8; 32], [u8; 32], [u8; 32])> {
    let dir = ProjectDirs::from("ch", "Timelock", "Client").unwrap();
    let mut file = File::create(dir.data_dir().join("credentials"))?;

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
