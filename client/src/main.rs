use std::error::Error;

mod crypto;

fn main() {
    let result = crypto::init();
    if result != 0 {
        panic!("Cryptography module initialization failed")
    }
    println!("Client started!");
}

/// Tries to read keys off of the default location.
fn read_keys() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), std::io::Error> {
    let master_key: Vec<u8> = vec![0, 32];
    let auth_key: Vec<u8> = vec![0, 32];
    let enc_key: Vec<u8> = vec![0, 32];
    let pub_key: Vec<u8> = vec![0, 32];
    let priv_key: Vec<u8> = vec![0, 32];

    Ok((master_key, auth_key, enc_key, pub_key, priv_key))
}

/// Creates the basic key suite
fn create_keys() -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>), std::io::Error> {
    let master_key: Vec<u8> = vec![0, 32];
    let auth_key: Vec<u8> = vec![0, 32];
    let enc_key: Vec<u8> = vec![0, 32];
    let pub_key: Vec<u8> = vec![0, 32];
    let priv_key: Vec<u8> = vec![0, 32];

    Ok((master_key, auth_key, enc_key, pub_key, priv_key))
}

fn unlock_message() {}

fn download_unlock_key() {}

fn download_message() {}

fn list_messages() {}

///
fn send_message() {}
