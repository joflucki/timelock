use libsodium_sys;

pub const NONCE_SIZE: usize = libsodium_sys::crypto_stream_xchacha20_NONCEBYTES as usize;
pub const MAC_SIZE: usize = libsodium_sys::crypto_auth_BYTES as usize;
pub const KEY_SIZE: usize = libsodium_sys::crypto_stream_xchacha20_KEYBYTES as usize;
pub const SALT_SIZE: usize = libsodium_sys::crypto_pwhash_SALTBYTES as usize;
