/*
use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ClientConnection, StreamOwned, ServerName};
use std::net::TcpStream;
use std::sync::Arc;
use std::io::Write;
use std::fs;
use anyhow::Result;

fn load_client_config(ca_cert_path: &str) -> Result<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    let cert_data = fs::read(ca_cert_path)?;
    let certs = rustls_pemfile::certs(&mut &*cert_data)?;

    root_store.add_parsable_certificates(&certs);

    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    Ok(config)
}

fn send_tls_message(domain: &str, addr: &str, config: ClientConfig, message: &SecureMessage) -> Result<()> {
    let tcp_stream = TcpStream::connect(addr)?;
    let server_name = ServerName::try_from(domain)?;
    let mut tls_conn = ClientConnection::new(Arc::new(config), server_name)?;
    let mut tls_stream = StreamOwned::new(tls_conn, tcp_stream);

    let encoded = bincode::serialize(message)?;
    tls_stream.write_all(&(encoded.len() as u32).to_be_bytes())?;
    tls_stream.write_all(&encoded)?;

    println!("Sent: {:?}", message);
    Ok(())
}


*/