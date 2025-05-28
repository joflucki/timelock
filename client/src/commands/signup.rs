use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use shared::frames::{ClientFrame, ServerFrame};

pub fn signup(username: &String) -> Result<()> {
    if !shared::utils::is_username_valid(&username) {
        return Err(anyhow!(
            "Username should contain only lowercase alphanumerical characters"
        ));
    }

    // Prompt for password
    let password = rpassword::prompt_password("Your password: ")?.to_string();

    // Generate salt
    let mut salt: [u8; SALT_SIZE] = [0; SALT_SIZE];
    random_buffer(&mut salt)?;

    // Generate root keys
    let mut master_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    hash_password(&mut master_key, &password, &salt)?;

    // Derive auth key and enc key from master key
    let auth_context: &'static str = "Authentication";
    let enc_context: &'static str = "Encryption";

    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut enc_key: [u8; KEY_SIZE] = [0; KEY_SIZE];

    derive_key(&master_key, &mut auth_key, auth_context)?;
    derive_key(&master_key, &mut enc_key, enc_context)?;

    // Generate keypair
    let mut private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut public_key: [u8; KEY_SIZE] = [0; KEY_SIZE];

    generate_keypair(&mut public_key, &mut private_key)?;

    // Generate nonce
    let mut nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    random_buffer(&mut nonce)?;

    // Encrypt private key
    let mut encrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_encrypt(&nonce, &private_key, &enc_key, &mut encrypted_private_key)?;

    // Connect to the server
    let mut stream = network::connect()?;

    // Send identify message
    network::write(
        &mut stream,
        ClientFrame::Identify {
            username: username.clone(),
            public_key,
            auth_key,
            encrypted_private_key: encrypted_private_key,
            salt,
            nonce: nonce,
        },
    )?;

    match network::read(&mut stream)? {
        ServerFrame::IdentifyResponse {} => {}
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected server response")),
    };

    utils::save_keys(&master_key, &auth_key, &enc_key, &private_key, &public_key)?;
    utils::save_username(username)?;
    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;
    Ok(())
}
