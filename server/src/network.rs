use anyhow::Result;
use native_tls::TlsStream;
use shared::frames::{ClientFrame, ServerFrame};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn write(stream: &mut TlsStream<TcpStream>, frame: ServerFrame) -> Result<()> {
    let mut encoded = bincode::serialize(&frame)?;
    let mut length = (encoded.len() as u32).to_be_bytes();
    stream.write_all(&mut length)?;
    stream.write_all(&mut encoded)?;
    Ok(())
}

pub fn read(stream: &mut TlsStream<TcpStream>) -> Result<ClientFrame> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let frame: ClientFrame = bincode::deserialize(&buffer)?;
    Ok(frame)
}
