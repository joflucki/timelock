/// This module defines the network messages exchanged between the client and server.
use crate::{crypto::*, models::MessagePreview};
use serde::{Deserialize, Serialize};

/// Client network frame sent to the server
#[derive(Serialize, Deserialize, Debug)]
pub enum ClientFrame {
    Identify {
        username: String,
        public_key: [u8; KEY_SIZE],
        auth_key: [u8; KEY_SIZE],
        encrypted_private_key: [u8; KEY_SIZE],
        salt: [u8; SALT_SIZE],
        nonce: [u8; NONCE_SIZE],
    },
    GetSalt {
        username: String,
    },
    GetCredentials {
        username: String,
        auth_key: [u8; KEY_SIZE],
    },
    ResetPassword {
        username: String,
        auth_key: [u8; KEY_SIZE],
        encrypted_private_key: [u8; KEY_SIZE],
        salt: [u8; SALT_SIZE],
        nonce: [u8; NONCE_SIZE],
        mac: [u8; MAC_SIZE],
    },
    GetPublicKey {
        username: String,
    },
    SendMessage {
        sender_username: String,
        recipient_username: String,
        timestamp: [u8; 8],
        encrypted_key: [u8; KEY_SIZE],
        key_nonce: [u8; NONCE_SIZE],
        key_mac: [u8; MAC_SIZE],
        encrypted_data: Vec<u8>,
        data_nonce: [u8; NONCE_SIZE],
        data_mac: [u8; MAC_SIZE],
        mac: [u8; MAC_SIZE],
    },
    ListMessages {
        username: String,
        mac: [u8; MAC_SIZE],
    },
    ListUsers {},
    DownloadMessage {
        username: String,
        message_id: String,
        mac: [u8; MAC_SIZE],
    },
    UnlockMessage {
        username: String,
        message_id: String,
        mac: [u8; MAC_SIZE],
    },
}

/// Server network frame sent to the client
#[derive(Serialize, Deserialize, Debug)]
pub enum ServerFrame {
    IdentifyResponse {
        ok: bool,
        server_public_key: [u8; KEY_SIZE],
    },
    GetSaltResponse {
        salt: [u8; SALT_SIZE],
    },
    GetCredentialsResponse {
        server_public_key: [u8; KEY_SIZE],
        encrypted_private_key: [u8; KEY_SIZE],
        nonce: [u8; NONCE_SIZE],
    },
    ResetPasswordResponse {
        ok: bool,
    },
    GetPublicKeyResponse {
        public_key: [u8; KEY_SIZE],
    },
    SendMessageResponse {
        ok: bool,
    },
    ListMessagesResponse {
        message_previews: Vec<MessagePreview>,
    },
    ListUsersResponse {
        usernames: Vec<String>,
    },
    DownloadMessageResponse {
        sender_public_key: [u8; KEY_SIZE],
        encrypted_data: Vec<u8>,
        data_nonce: [u8; KEY_SIZE],
        data_mac: [u8; MAC_SIZE],
    },
    UnlockMessageResponse {
        sender_public_key: [u8; KEY_SIZE],
        encrypted_key: [u8; KEY_SIZE],
        key_nonce: [u8; NONCE_SIZE],
        key_mac: [u8; MAC_SIZE],
    },
}
