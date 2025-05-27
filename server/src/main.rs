mod handlers;

use anyhow::{anyhow, Result};
use chrono::DateTime;
use handlers::*;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() -> Result<()> {
    shared::crypto::init()?;

    let identity =
        Identity::from_pkcs8(include_bytes!("server.pem"), include_bytes!("server.key"))?;
    let listener = TcpListener::bind("0.0.0.0:8443")?;
    let acceptor = TlsAcceptor::new(identity)?;
    let acceptor = Arc::new(acceptor);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut stream = acceptor
                        .accept(stream)
                        .expect("Could not accept connection");
                    match handle_client(&mut stream) {
                        Err(e) => eprintln!("{}", e),
                        _ => {}
                    }
                });
            }
            Err(e) => return Err(anyhow!(e)),
        }
    }
    Ok(())
}

fn handle_client(stream: &mut TlsStream<TcpStream>) -> Result<()> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let message: shared::frames::ClientFrame = bincode::deserialize(&buffer)?;

    match message {
        shared::frames::ClientFrame::GetCredentials { username, auth_key } => {
            get_credentials(stream, &username, &auth_key)
        }
        shared::frames::ClientFrame::Identify {
            username,
            public_key,
            auth_key,
            encrypted_private_key,
            salt,
            nonce,
        } => identify(
            stream,
            &username,
            &public_key,
            &auth_key,
            &encrypted_private_key,
            &salt,
            &nonce,
        ),
        shared::frames::ClientFrame::GetSalt { username } => get_salt(stream, &username),
        shared::frames::ClientFrame::ResetPassword {
            username,
            auth_key,
            encrypted_private_key,
            salt,
            nonce,
            mac,
        } => reset_password(
            stream,
            &username,
            &auth_key,
            &encrypted_private_key,
            &salt,
            &nonce,
            &mac,
        ),
        shared::frames::ClientFrame::GetPublicKey { username } => get_public_key(stream, &username),
        shared::frames::ClientFrame::SendMessage {
            sender_username,
            recipient_username,
            timestamp,
            encrypted_key,
            key_nonce,
            key_mac,
            encrypted_data,
            data_nonce,
            data_mac,
            mac,
        } => send_message(
            stream,
            &sender_username,
            &recipient_username,
            &DateTime::from_timestamp(i64::from_be_bytes(timestamp), 0).unwrap(),
            &encrypted_key,
            &key_nonce,
            &key_mac,
            encrypted_data,
            &data_nonce,
            &data_mac,
            &mac,
        ),
        shared::frames::ClientFrame::ListMessages { username, mac } => {
            list_messages(stream, &username, &mac)
        }
        shared::frames::ClientFrame::DownloadMessage {
            username,
            message_id,
            mac,
        } => download_message(stream, &username, &message_id, &mac),
        shared::frames::ClientFrame::UnlockMessage {
            username,
            message_id,
            mac,
        } => unlock_message(stream, &username, &message_id, &mac),
        shared::frames::ClientFrame::ListUsers {} => list_users(stream),
    }
}
