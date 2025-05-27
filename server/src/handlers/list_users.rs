use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

pub fn list_users(stream: &mut TlsStream<TcpStream>) -> Result<()> {
    todo!()
}
