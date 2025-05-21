//! This module wraps libsodium bindings
//! and offers a pragmatic Rust API for cryptography
//! related tasks.

use libc;
use libsodium_sys;
use std::ffi::CString;

pub fn init() -> i32 {
    unsafe { libsodium_sys::sodium_init() }
}

pub fn generate_keypair(public_key: &mut [u8; 32], private_key: &mut [u8; 32]) {
    let ret = unsafe {
        libsodium_sys::crypto_box_keypair(public_key.as_mut_ptr(), private_key.as_mut_ptr())
    };
    if ret != 0 {
        panic!("Keypair generation failed");
    }
}

/// Derives a subkey from a master key.
pub fn derive_key(master_key: &[u8; 32], sub_key: &mut [u8; 32], context: &str) {
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

pub fn hash_password(hash: &mut [u8; 32], password: &str, salt: &[u8; 32]) {
    let c_password = CString::new(password).expect("CString::new failed");
    let ret = unsafe {
        libsodium_sys::crypto_pwhash(
            hash.as_mut_ptr(),
            hash.len() as u64,
            c_password.as_ptr(),
            password.len() as u64,
            salt.as_ptr(),
            100,
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
    nonce: &[u8; 24],
    plaintext: &[u8],
    key: &[u8; 32],
    ciphertext: &mut [u8],
) {
    let ret = unsafe {
        libsodium_sys::crypto_secretbox_easy(
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
