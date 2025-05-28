use anyhow::Result;
use native_tls::TlsStream;
use shared::crypto::KEY_SIZE;
use std::net::TcpStream;

pub fn download_message(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    message_id: &str,
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    todo!()
}
