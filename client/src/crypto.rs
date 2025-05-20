use libc;
use libsodium_sys;
use std::ffi::CString;

pub fn init() -> i32 {
    unsafe { libsodium_sys::sodium_init() }
}

/// Derives the public key associated with a private key.
pub fn compute_public_key(private_key: Vec<u8>) -> Vec<u8> {
    let mut public_key: Vec<u8> = vec![0, 32];
    let result = unsafe {
        libsodium_sys::crypto_scalarmult_base(
            public_key.as_mut_ptr() as *mut libc::c_uchar,
            private_key.as_ptr() as *const libc::c_uchar,
        )
    };
    if result != 0 {
        panic!("Key derivation failed");
    }
    public_key
}

/// Generates a symmetric key.
pub fn generate_symmetric_key() -> Vec<u8> {
    let mut private_key: Vec<u8> = vec![0, 32];
    unsafe {
        libsodium_sys::crypto_secretstream_xchacha20poly1305_keygen(
            private_key.as_mut_ptr() as *mut libc::c_uchar
        );
    }
    private_key
}

/// Derives a sub key from a master key.
pub fn derive_key(prk: &[u8], context: &str, key_len: usize) -> Vec<u8> {
    assert_eq!(
        prk.len(),
        libsodium_sys::crypto_kdf_hkdf_sha256_KEYBYTES as usize
    );
    let mut subkey: Vec<u8> = vec![0u8; key_len];
    let ctx_cstr = CString::new(context).expect("Invalid context string");

    let result = unsafe {
        libsodium_sys::crypto_kdf_hkdf_sha256_expand(
            subkey.as_mut_ptr(),
            subkey.len(),
            ctx_cstr.as_ptr(),
            context.len(),
            prk.as_ptr(),
        )
    };

    if result != 0 {
        panic!("Key derivation failed");
    }

    subkey
}
