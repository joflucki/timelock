use anyhow::Result;
use native_tls::TlsStream;
use shared::crypto::MAC_SIZE;
use std::net::TcpStream;

pub fn download_message(
    stream: &mut TlsStream<TcpStream>,
    username: &str,
    message_id: &str,
    mac: &[u8; MAC_SIZE],
) -> Result<()> {
    todo!()
}
