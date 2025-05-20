use libc;
use libsodium_sys;

/// Generates an asymmetric keypair.
pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    let mut public_key: Vec<u8> = vec![0; 32];
    let mut private_key: Vec<u8> = vec![0; 32];

    let result = unsafe {
        libsodium_sys::crypto_box_curve25519xchacha20poly1305_keypair(
            public_key.as_mut_ptr() as *mut libc::c_uchar,
            private_key.as_mut_ptr() as *mut libc::c_uchar,
        )
    };
    if result != 0 {
        panic!("Key pair generation failed");
    }

    (public_key, private_key)
}