/// This module defines the messages exchanged between the client and server.
use serde::{Deserialize, Serialize};

/// Client messages sent to the server
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Identify {
        user_id: String,
        public_key: [u8; 32],
        auth_key: [u8; 32],
        encrypted_private_key: [u8; 32],
        salt: [u8; 32],
        nonce: [u8; 24],
    },
    GetSalt {
        user_id: String,
    },
    GetCredentials {
        user_id: String,
        auth_key: [u8; 32],
    },
    ResetPassword {
        user_id: String,
        auth_key: [u8; 32],
        encrypted_private_key: [u8; 32],
        salt: [u8; 32],
        nonce: [u8; 24],
        tag: [u8; 32],
    },
    GetPublicKey {
        id: String,
    },
    SendMessage {
        sender_id: String,
        receiver_id: String,
        timestamp: u64,
        encrypted_key: [u8; 32],
        key_nonce: [u8; 24],
        key_tag: [u8; 32],
        encrypted_message: Vec<u8>,
        message_nonce: [u8; 24],
        message_tag: [u8; 32],
        tag: [u8; 32],
    },
    ListMessages {
        user_id: String,
        tag: [u8; 32],
    },
    DownloadMessage {
        user_id: String,
        message_id: String,
        tag: [u8; 32],
    },
    UnlockMessage {
        user_id: String,
        message_id: String,
        tag: [u8; 32],
    },
}

/// Server messages sent to the client
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    IdentifiyResponse {
        ok: bool,
    },
    GetSaltResponse {
        salt: [u8; 32],
    },
    GetCredentialsResponse {
        public_key: [u8; 32],
        encrypted_private_key: [u8; 32],
        nonce: [u8; 24],
    },
    ResetPasswordResponse {
        ok: bool,
    },
    GetPublicKeyResponse {
        public_key: [u8; 32],
    },
    SendMessageResponse {
        ok: bool,
    },
    ListMessagesResponse {
        messages: Vec<String>, // TODO: Change to a more complete type
    },
    DownloadMessageResponse {
        encrypted_message: Vec<u8>,
        nonce: [u8; 24],
        tag: [u8; 32],
    },
    UnlockMessageResponse {
        encrypted_key: [u8; 32],
        nonce: [u8; 24],
        tag: [u8; 32],
    },
}
