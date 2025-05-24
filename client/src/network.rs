use anyhow::Result;
use native_tls::{TlsConnector, TlsStream};
use shared::messages::{ClientMessage, ServerMessage};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn connect() -> Result<TlsStream<TcpStream>> {
    let cert = native_tls::Certificate::from_pem(include_bytes!("server.crt")).unwrap();
    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();

    let stream = TcpStream::connect("localhost:8443").unwrap();
    Ok(connector.connect("timelock.ch", stream).unwrap())
}

pub fn write(stream: &mut TlsStream<TcpStream>, message: ClientMessage) -> Result<()> {
    let mut encoded = bincode::serialize(&message)?;
    let mut length = (encoded.len() as u32).to_be_bytes();
    stream.write_all(&mut length)?;
    stream.write_all(&mut encoded)?;
    Ok(())
}

pub fn read(stream: &mut TlsStream<TcpStream>) -> Result<ServerMessage> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let message: ServerMessage = bincode::deserialize(&buffer)?;
    Ok(message)
}
