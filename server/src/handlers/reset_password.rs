use crate::{network, utils};
use anyhow::{anyhow, Result};
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;
use subtle::ConstantTimeEq;

pub fn reset_password(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    new_auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    salt: &[u8; SALT_SIZE],
    nonce: &[u8; NONCE_SIZE],
    old_auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let (stored_auth_key, _, public_key, _, _) = utils::load_credentials(username)?;
    if !bool::from(stored_auth_key.ct_eq(old_auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    utils::save_credentials(
        username,
        new_auth_key,
        encrypted_private_key,
        &public_key,
        nonce,
        salt,
    )?;

    network::write(
        stream,
        shared::frames::ServerFrame::ResetPasswordResponse {},
    )?;

    Ok(())
}
