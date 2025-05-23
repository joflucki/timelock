use anyhow::Result;
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use shared::messages::{ClientMessage, ServerMessage};
use std::fs;
use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

// I have no idea what's going on here
fn load_client_config(ca_cert_path: &str) -> Result<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    let cert_data = fs::read(ca_cert_path)?;
    let mut binding = &*cert_data;
    let certs = rustls_pemfile::certs(&mut binding);
    root_store.add_parsable_certificates(certs.map(|cert| cert.unwrap()));
    let config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(config)
}

pub fn connect(
    addr: &str,
    domain: &str,
    ca_cert_path: &str,
) -> Result<StreamOwned<ClientConnection, TcpStream>> {
    let config = load_client_config(ca_cert_path)?;
    Ok(StreamOwned::new(
        ClientConnection::new(
            Arc::new(config),
            ServerName::try_from(domain.to_string()).expect("Invalid DNS name"),
        )?,
        TcpStream::connect(addr)?,
    ))
}

pub fn write(
    stream: &mut StreamOwned<ClientConnection, TcpStream>,
    message: ClientMessage,
) -> Result<()> {
    let encoded = bincode::serialize(&message)?;
    stream.write_all(&(encoded.len() as u32).to_be_bytes())?;
    stream.write_all(&encoded)?;
    Ok(())
}

pub fn read(stream: &mut StreamOwned<ClientConnection, TcpStream>) -> Result<ServerMessage> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length);
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let message: ServerMessage = bincode::deserialize(&buffer)?;
    Ok(message)
}
