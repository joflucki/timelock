use anyhow::Result;
use native_tls::{TlsConnector, TlsStream};
use shared::frames::{ClientFrame, ServerFrame};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

/// Connects to the server using TLS.
pub fn connect() -> Result<TlsStream<TcpStream>> {
    let cert = native_tls::Certificate::from_pem(include_bytes!("cert.pem"))?;
    let connector = TlsConnector::builder().add_root_certificate(cert).build()?;

    let stream = TcpStream::connect("localhost:8443")?;
    Ok(connector.connect("timelock.ch", stream)?)
}

/// Writes a network frame to the server.
pub fn write(stream: &mut TlsStream<TcpStream>, frame: ClientFrame) -> Result<()> {
    let mut encoded = bincode::serialize(&frame)?;
    let mut length = (encoded.len() as u32).to_be_bytes();
    stream.write_all(&mut length)?;
    stream.write_all(&mut encoded)?;
    Ok(())
}

/// Read a network frame from the server.
pub fn read(stream: &mut TlsStream<TcpStream>) -> Result<ServerFrame> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let frame: ServerFrame = bincode::deserialize(&buffer)?;
    Ok(frame)
}
