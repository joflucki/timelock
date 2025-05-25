//! This module wraps libsodium bindings
//! and offers a pragmatic Rust API for cryptographic operations.
use libc;
use libsodium_sys;
use shared::crypto::*;
use std::ffi::CString;

pub fn init() -> i32 {
    unsafe { libsodium_sys::sodium_init() }
}

pub fn generate_keypair(public_key: &mut [u8; KEY_SIZE], private_key: &mut [u8; KEY_SIZE]) {
    let ret = unsafe {
        libsodium_sys::crypto_box_keypair(public_key.as_mut_ptr(), private_key.as_mut_ptr())
    };
    if ret != 0 {
        panic!("Keypair generation failed");
    }
}

pub fn derive_public_key(public_key: &mut [u8; KEY_SIZE], private_key: &[u8; KEY_SIZE]) {
    let ret = unsafe {
        libsodium_sys::crypto_scalarmult_base(public_key.as_mut_ptr(), private_key.as_ptr())
    };
    if ret != 0 {
        panic!("Public key derivation failed");
    }
}

/// Derives a subkey from a master key.
pub fn derive_key(master_key: &[u8; KEY_SIZE], sub_key: &mut [u8; KEY_SIZE], context: &str) {
    let ctx_cstr = CString::new(context).expect("Invalid context string");

    let result = unsafe {
        libsodium_sys::crypto_kdf_hkdf_sha256_expand(
            sub_key.as_mut_ptr(),
            32,
            ctx_cstr.as_ptr(),
            context.len(),
            master_key.as_ptr(),
        )
    };

    if result != 0 {
        panic!("Key derivation failed");
    }
}

/// Hashes a password into a key.
pub fn hash_password(hash: &mut [u8; KEY_SIZE], password: &str, salt: &[u8; SALT_SIZE]) {
    let c_password = CString::new(password).expect("CString::new failed");
    let ret = unsafe {
        libsodium_sys::crypto_pwhash(
            hash.as_mut_ptr(),
            hash.len() as u64,
            c_password.as_ptr(),
            password.len() as u64,
            salt.as_ptr(),
            50,
            libsodium_sys::crypto_pwhash_MEMLIMIT_INTERACTIVE as usize,
            libsodium_sys::crypto_pwhash_ALG_DEFAULT as i32,
        )
    };
    if ret != 0 {
        panic!("Password hashing failed");
    }
}

pub fn random_buffer(buffer: &mut [u8]) {
    unsafe {
        libsodium_sys::randombytes_buf(buffer.as_mut_ptr() as *mut libc::c_void, buffer.len());
    }
}

pub fn symmetric_encrypt(
    nonce: &[u8; NONCE_SIZE],
    plaintext: &[u8],
    key: &[u8; KEY_SIZE],
    ciphertext: &mut [u8],
) {
    let ret = unsafe {
        libsodium_sys::crypto_stream_xchacha20_xor(
            ciphertext.as_mut_ptr(),
            plaintext.as_ptr(),
            plaintext.len() as u64,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    };
    if ret != 0 {
        panic!("Encryption failed");
    }
}

pub fn symmetric_decrypt(
    nonce: &[u8; NONCE_SIZE],
    ciphertext: &[u8],
    key: &[u8; KEY_SIZE],
    plaintext: &mut [u8],
) {
    let ret = unsafe {
        libsodium_sys::crypto_stream_xchacha20_xor(
            plaintext.as_mut_ptr(),
            ciphertext.as_ptr(),
            ciphertext.len() as u64,
            nonce.as_ptr(),
            key.as_ptr(),
        )
    };
    if ret != 0 {
        panic!("Decryption failed");
    }
}

pub fn authenticate(mac: &mut [u8; MAC_SIZE], key: &[u8; KEY_SIZE], message: &[u8]) {
    let ret = unsafe {
        libsodium_sys::crypto_auth(
            mac.as_mut_ptr(),
            message.as_ptr(),
            message.len() as u64,
            key.as_ptr(),
        )
    };
    if ret != 0 {
        panic!("Authentication failed");
    }
}
