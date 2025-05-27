use anyhow::Result;
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;

pub fn identify(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    public_key: &[u8; KEY_SIZE],
    auth_key: &[u8; KEY_SIZE],
    encrypted_private_key: &[u8; KEY_SIZE],
    salt: &[u8; SALT_SIZE],
    nonce: &[u8; NONCE_SIZE],
) -> Result<()> {
    todo!()
}
