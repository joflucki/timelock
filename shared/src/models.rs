use serde::{Deserialize, Serialize};

use crate::crypto::{KEY_SIZE, MAC_SIZE, NONCE_SIZE};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessagePreview {
    pub message_id: String,
    pub sender_username: String,
    pub unlock_timestamp: u64,
    pub file_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageMetadataFile {
    pub sender_username: String,
    pub unlock_timestamp: u64,
    pub encrypted_key: [u8; KEY_SIZE],
    pub key_nonce: [u8; NONCE_SIZE],
    pub key_mac: [u8; MAC_SIZE],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDataFile {
    pub encrypted_data: Vec<u8>,
    pub data_nonce: [u8; NONCE_SIZE],
    pub data_mac: [u8; MAC_SIZE],
}
