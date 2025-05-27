use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

pub fn get_salt(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
    todo!()
}
