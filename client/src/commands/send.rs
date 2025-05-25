use crate::utils;
use std::path::Path;

pub fn send(filepath: &Path, recipient_username: &String) {
    let (master_key, auth_key, enc_key, private_key, public_key, server_public_key) =
        utils::load_keys().expect("Error loading keys");
}
