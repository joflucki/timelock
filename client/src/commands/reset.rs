use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use shared::frames::ClientFrame;

/// Resets the user's password and updates local authentication files.
/// 
/// Requires prior authentication.
pub fn reset() -> Result<()> {
    // Load username
    let username = utils::load_username()?;

    // Prompt for password
    let password = rpassword::prompt_password("New password: ")?.to_string();

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

    // Generate nonce
    let mut nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    random_buffer(&mut nonce)?;

    // Load keys
    let (_, old_auth_key, _, private_key, public_key) = utils::load_keys()?;

    // Encrypt private key
    let mut encrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_encrypt(&nonce, &private_key, &enc_key, &mut encrypted_private_key)?;

    // Connect to the server
    let mut stream = network::connect()?;

    // Send the reset password message
    network::write(
        &mut stream,
        ClientFrame::ResetPassword {
            username: username.clone(),
            new_auth_key: auth_key,
            encrypted_private_key: encrypted_private_key,
            salt: salt,
            nonce: nonce,
            old_auth_key: old_auth_key,
        },
    )?;

    // Read the response from the server
    match network::read(&mut stream)? {
        shared::frames::ServerFrame::ResetPasswordResponse {} => {}
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    utils::save_keys(&master_key, &auth_key, &enc_key, &private_key, &public_key)?;
    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;
    Ok(())
}
