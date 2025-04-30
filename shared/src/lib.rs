mod messages;
use libc::c_uchar;
pub use messages::*;

use libsodium_sys;

pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    // Allocate memory for the public and secret keys
    let mut pk: Vec<u8> = vec![0; 32]; // Public key size
    let mut sk: Vec<u8> = vec![0; 32]; // Secret key size

    // Call the unsafe function to generate the key pair
    unsafe {
        let result = libsodium_sys::crypto_box_curve25519xchacha20poly1305_keypair(
            pk.as_mut_ptr() as *mut c_uchar,
            sk.as_mut_ptr() as *mut c_uchar,
        );

        if result != 0 {
            panic!("Key pair generation failed");
        }
    }

    (pk, sk)
}
