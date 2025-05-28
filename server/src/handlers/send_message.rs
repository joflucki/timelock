use anyhow::Result;
use chrono::{DateTime, Utc};
use native_tls::TlsStream;
use shared::crypto::*;
use std::net::TcpStream;

pub fn send_message(
    stream: &mut TlsStream<TcpStream>,
    sender_username: &str,
    recipient_username: &str,
    datetime: &DateTime<Utc>,
    encrypted_key: &[u8; KEY_SIZE],
    key_nonce: &[u8; NONCE_SIZE],
    key_mac: &[u8; MAC_SIZE],
    encrypted_data: Vec<u8>,
    data_nonce: &[u8; NONCE_SIZE],
    data_mac: &[u8; MAC_SIZE],
    auth_key: &[u8; KEY_SIZE],
) -> Result<()> {
    todo!()
}
