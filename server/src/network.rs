use anyhow::Result;
use native_tls::TlsStream;
use shared::frames::ServerFrame;
use std::{io::Write, net::TcpStream};

pub fn write(stream: &mut TlsStream<TcpStream>, frame: ServerFrame) -> Result<()> {
    let mut encoded = bincode::serialize(&frame)?;
    let mut length = (encoded.len() as u32).to_be_bytes();
    stream.write_all(&mut length)?;
    stream.write_all(&mut encoded)?;
    Ok(())
}
