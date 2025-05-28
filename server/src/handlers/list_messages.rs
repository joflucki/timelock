use anyhow::Result;
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;

pub fn list_messages(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    todo!()
}
