use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

pub fn get_public_key(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
    todo!()
}
