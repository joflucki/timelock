use anyhow::{anyhow, Result};
use libsodium_sys;

pub const NONCE_SIZE: usize = libsodium_sys::crypto_stream_xchacha20_NONCEBYTES as usize;
pub const MAC_SIZE: usize = libsodium_sys::crypto_auth_BYTES as usize;
pub const KEY_SIZE: usize = libsodium_sys::crypto_stream_xchacha20_KEYBYTES as usize;
pub const SALT_SIZE: usize = libsodium_sys::crypto_pwhash_SALTBYTES as usize;

pub fn init() -> Result<()> {
    let ret = unsafe { libsodium_sys::sodium_init() };
    match ret {
        0 => Ok(()),
        _ => Err(anyhow!("Initialization of cryptographic module failed")),
    }
}

pub fn verify_authentication(
    mac: &[u8; MAC_SIZE],
    key: &[u8; KEY_SIZE],
    message: &[u8],
) -> Result<bool> {
    let ret = unsafe {
        libsodium_sys::crypto_auth_verify(
            mac.as_ptr(),
            message.as_ptr(),
            message.len() as u64,
            key.as_ptr(),
        )
    };
    Ok(ret == 0)
}

pub fn exchange_keys(
    public_key: &[u8; 32],
    private_key: &[u8; 32],
    shared_key: &mut [u8; 32],
) -> Result<()> {
    let ret = unsafe {
        libsodium_sys::crypto_scalarmult(
            shared_key.as_mut_ptr(),
            private_key.as_ptr(),
            public_key.as_ptr(),
        )
    };

    match ret {
        0 => Ok(()),
        _ => Err(anyhow!("Key exchange failed")),
    }
}
