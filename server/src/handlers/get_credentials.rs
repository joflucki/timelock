use crate::{network, utils};
use anyhow::{anyhow, Result};
use native_tls::TlsStream;
use shared::crypto::KEY_SIZE;
use std::net::TcpStream;
use subtle::ConstantTimeEq;

pub fn get_credentials(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    let (stored_auth_key, encrypted_private_key, _, nonce, _) = utils::load_credentials(username)?;

    if !bool::from(auth_key.ct_eq(&stored_auth_key)) {
        return Err(anyhow!("Authentication invalid"));
    }

    network::write(
        stream,
        shared::frames::ServerFrame::GetCredentialsResponse {
            encrypted_private_key: encrypted_private_key,
            nonce: nonce,
        },
    )?;

    Ok(())
}
