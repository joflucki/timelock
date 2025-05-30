use crate::crypto::*;
use crate::network;
use crate::utils;
use anyhow::{anyhow, Result};
use shared::crypto::*;
use shared::frames::{ClientFrame, ServerFrame};

/// Authenticates an existing user with its username and password.
///
/// Credentials are then saved on a local file for future commands.
pub fn login(username: &String) -> Result<()> {
    // Connect to the server
    let mut stream = network::connect()?;

    // Get salt
    network::write(
        &mut stream,
        ClientFrame::GetSalt {
            username: username.clone(),
        },
    )?;
    let salt = match network::read(&mut stream)? {
        ServerFrame::GetSaltResponse { salt } => salt,
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    // Prompt for password
    let password = rpassword::prompt_password("Your password: ")?.to_string();

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

    // Send to server
    network::write(
        &mut stream,
        ClientFrame::GetCredentials {
            username: username.clone(),
            auth_key,
        },
    )?;
    let (encrypted_private_key, nonce) = match network::read(&mut stream)? {
        ServerFrame::GetCredentialsResponse {
            encrypted_private_key,
            nonce,
        } => (encrypted_private_key, nonce),
        shared::frames::ServerFrame::Error { message } => return Err(anyhow!(message)),
        _ => return Err(anyhow!("Unexpected response from server")),
    };

    // Derive auth key from master key
    let auth_context: &'static str = "Authentication";
    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    derive_key(&master_key, &mut auth_key, auth_context)?;

    // Decrypt private key
    let mut decrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_decrypt(
        &nonce,
        &encrypted_private_key,
        &enc_key,
        &mut decrypted_private_key,
    )?;

    // Compute public key from private key
    let mut public_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    derive_public_key(&mut public_key, &decrypted_private_key)?;

    utils::save_keys(
        &master_key,
        &auth_key,
        &enc_key,
        &decrypted_private_key,
        &public_key,
    )?;
    utils::save_username(username)?;

    network::write(&mut stream, shared::frames::ClientFrame::Disconnect {})?;

    println!("Login successful");
    println!("You are now authenticated as {}", username);
    Ok(())
}
