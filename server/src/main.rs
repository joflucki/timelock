use anyhow::Result;
use native_tls::{Identity, TlsAcceptor, TlsStream};
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

fn main() {
    let identity =
        Identity::from_pkcs8(include_bytes!("server.pem"), include_bytes!("server.key")).unwrap();

    let listener = TcpListener::bind("0.0.0.0:8443").unwrap();
    let acceptor = TlsAcceptor::new(identity).unwrap();
    let acceptor = Arc::new(acceptor);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut stream = acceptor.accept(stream).unwrap();
                    handle_client(&mut stream).unwrap();
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_client(stream: &mut TlsStream<TcpStream>) -> Result<()> {
    let mut length: [u8; 4] = [0; 4];
    stream.read_exact(&mut length)?;
    let length = u32::from_be_bytes(length);
    let mut buffer = vec![0; length as usize];
    stream.read_exact(&mut buffer)?;
    let message: shared::messages::ClientMessage =
        bincode::deserialize(&buffer).expect("Failed to deserialize message");

    match message {
        shared::messages::ClientMessage::GetCredentials { username, auth_key } => Ok(()),
        shared::messages::ClientMessage::Identify {
            username,
            public_key,
            auth_key,
            encrypted_private_key,
            salt,
            nonce,
        } => todo!(),
        shared::messages::ClientMessage::GetSalt { username } => {
            println!("GetSalt request for username: {}", username);
            Ok(())
        }
        shared::messages::ClientMessage::ResetPassword {
            username,
            auth_key,
            encrypted_private_key,
            salt,
            nonce,
            mac,
        } => todo!(),
        shared::messages::ClientMessage::GetPublicKey { id } => todo!(),
        shared::messages::ClientMessage::SendMessage {
            sender_username,
            recipient_username,
            timestamp,
            encrypted_key,
            key_nonce,
            key_mac,
            encrypted_message,
            message_nonce,
            message_mac,
            mac,
        } => todo!(),
        shared::messages::ClientMessage::ListMessages { username, mac } => todo!(),
        shared::messages::ClientMessage::DownloadMessage {
            username,
            message_id,
            mac,
        } => todo!(),
        shared::messages::ClientMessage::UnlockMessage {
            username,
            message_id,
            mac: mac,
        } => todo!(),
    }
}
