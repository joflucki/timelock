mod app;
mod crypto;
use app::*;
use crypto::*;

fn main() {
    let app = App::new();
    app.run();
}
fn generate_keys() {
    let result = crypto::init();
    if result != 0 {
        panic!("Cryptography module initialization failed")
    }
    println!("Client started!");

    // Derive master key from password
    let password: &'static str = "password";
    let mut master_key: [u8; 32] = [0; 32];
    let mut salt: [u8; 32] = [0; 32];

    random_buffer(&mut salt);
    println!("Salt:\t\t\t{:?}", salt);

    hash_password(&mut master_key, password, &salt);
    println!("Master key:\t\t{:?}", master_key);

    // Derive subkeys from master key
    let auth_context: &'static str = "Authentication";
    let enc_context: &'static str = "Encryption";
    let mut auth_key: [u8; 32] = [0; 32];
    let mut enc_key: [u8; 32] = [0; 32];

    derive_key(&master_key, &mut auth_key, auth_context);
    derive_key(&master_key, &mut enc_key, enc_context);

    println!("Authentication key:\t{:?}", auth_key);
    println!("Encryption key:\t\t{:?}", enc_key);

    // Generate keypair
    let mut public_key: [u8; 32] = [0; 32];
    let mut private_key: [u8; 32] = [0; 32];

    generate_keypair(&mut public_key, &mut private_key);

    println!("Public key:\t\t{:?}", public_key);
    println!("Private key:\t\t{:?}", private_key);

    // Encrypt private key with encryption key
    let mut encrypted_private_key: [u8; 32] = [0; 32];
    let mut nonce: [u8; 24] = [0; 24];

    random_buffer(&mut nonce);
    println!("Nonce:\t\t\t{:?}", nonce);

    symmetric_encrypt(&nonce, &private_key, &enc_key, &mut encrypted_private_key);
    println!("Encrypted private key:\t{:?}", encrypted_private_key);
}
