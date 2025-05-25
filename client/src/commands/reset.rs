use crate::crypto::*;
use crate::network;
use crate::utils;
use shared::crypto::*;
use shared::messages::ClientMessage;

pub fn reset(username: &String) {
    // Prompt for password
    let password = rpassword::prompt_password("New password: ")
        .unwrap()
        .to_string();

    // Generate salt
    let mut salt: [u8; SALT_SIZE] = [0; SALT_SIZE];
    random_buffer(&mut salt);

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

    // Generate nonce
    let mut nonce: [u8; NONCE_SIZE] = [0; NONCE_SIZE];
    random_buffer(&mut nonce);

    // Load keys
    let (_, _, _, private_key, public_key, server_public_key) = utils::load_keys().unwrap();

    // Encrypt private key
    let mut encrypted_private_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    symmetric_encrypt(&nonce, &private_key, &enc_key, &mut encrypted_private_key);

    // Compute shared key
    let mut shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut shared_key);

    // Authenticate the message
    let mut message: Vec<u8> = Vec::new();
    message.extend_from_slice(username.as_bytes());
    message.extend_from_slice(&auth_key);
    message.extend_from_slice(&encrypted_private_key);
    message.extend_from_slice(&nonce);
    message.extend_from_slice(&salt);

    let mut mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut mac, &shared_key, &message);

    // Connect to the server
    let mut stream = network::connect().unwrap();

    // Send the reset password message
    network::write(
        &mut stream,
        ClientMessage::ResetPassword {
            username: username.clone(),
            auth_key: auth_key,
            encrypted_private_key: encrypted_private_key,
            salt: salt,
            nonce: nonce,
            mac: mac,
        },
    )
    .expect("Error sending reset password message to server");

    // Read the response from the server
    match network::read(&mut stream).expect("Error reading response from server") {
        shared::messages::ServerMessage::ResetPasswordResponse { ok } => {
            if ok {
                utils::save_keys(
                    &master_key,
                    &auth_key,
                    &enc_key,
                    &private_key,
                    &public_key,
                    &server_public_key,
                )
                .expect("Error saving new keys");
                println!("Password reset successfully.");
            } else {
                println!("Failed to reset password. Please try again.");
            }
        }
        _ => panic!("Unexpected response from server."),
    }
}
