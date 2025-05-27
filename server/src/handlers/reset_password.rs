use anyhow::Result;
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;

pub fn reset_password(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    salt: &[u8; SALT_SIZE],
    nonce: &[u8; NONCE_SIZE],
    mac: &[u8; MAC_SIZE],
) -> Result<()> {
    todo!()
}
