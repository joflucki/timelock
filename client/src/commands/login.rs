use crate::crypto::*;
use crate::network;
use crate::utils;
use shared::crypto::*;
use shared::messages::{ClientMessage, ServerMessage};

pub fn login(username: &String) {
    // Connect to the server
    let mut stream = network::connect().expect("Error connecting to server");

    // Get salt
    network::write(
        &mut stream,
        ClientMessage::GetSalt {
            username: username.clone(),
        },
    )
    .expect("Error sending salt request to server");
    let option = match network::read(&mut stream).expect("Error reading response from server") {
        ServerMessage::GetSaltResponse { salt } => Some(salt),
        _ => None,
    };
    let salt: [u8; SALT_SIZE] = option.unwrap();

    // Prompt for password
    let password = rpassword::prompt_password("Your password: ")
        .unwrap()
        .to_string();

    // Generate root keys
    let mut master_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    hash_password(&mut master_key, &password, &salt);

    // Derive auth key and enc key from master key
    let auth_context: &'static str = "Authentication";
    let enc_context: &'static str = "Encryption";

    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    let mut enc_key: [u8; KEY_SIZE] = [0; KEY_SIZE];

    derive_key(&master_key, &mut auth_key, auth_context);
    derive_key(&master_key, &mut enc_key, enc_context);

    // Send to server
    network::write(
        &mut stream,
        ClientMessage::GetCredentials {
            username: username.clone(),
            auth_key,
        },
    )
    .expect("Error sending credential request to server");
    let (server_public_key, encrypted_private_key, nonce) =
        match network::read(&mut stream).expect("Woopsies") {
            ServerMessage::GetCredentialsResponse {
                server_public_key,
                encrypted_private_key,
                nonce,
            } => Some((server_public_key, encrypted_private_key, nonce)),
            _ => todo!("Handle error: credentials not found"),
        }
        .expect("Error reading credentials from server");

    // Derive auth key from master key
    let auth_context: &'static str = "Authentication";
    let mut auth_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    derive_key(&master_key, &mut auth_key, auth_context);

    // Decrypt private key
    let mut decrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_decrypt(
        &nonce,
        &encrypted_private_key,
        &auth_key,
        &mut decrypted_private_key,
    );

    // Compute public key from private key
    let mut public_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    derive_public_key(&mut public_key, &decrypted_private_key);

    utils::save_keys(
        &master_key,
        &auth_key,
        &enc_key,
        &decrypted_private_key,
        &public_key,
        &server_public_key,
    )
    .unwrap();
}
