mod cli;
mod crypto;
mod network;

use clap::Parser;
use cli::*;
use crypto::*;
use shared::messages::{ClientMessage, ServerMessage};

fn main() {
    // Initialize the cryptography module.
    // If it fails, nothing works
    // and we should panic
    let ret = crypto::init();
    if ret != 0 {
        panic!("Cryptography module initialization failed")
    }

    // Parse command line arguments
    let cli = Cli::parse();
    match &cli.command {
        Commands::Login { username } => login(username),
        Commands::Send {
            filepath,
            recipient_username,
        } => send(filepath, recipient_username),
        Commands::List { list_command } => match list_command {
            ListCommands::Users => list_users(),
            ListCommands::Messages => list_messages(),
        },
        Commands::Signup { username, password } => signup(username, password),
        Commands::Reset {} => reset(),
        Commands::Download { filepath, file_id } => download(filepath, file_id),
        Commands::Unlock { filepath, file_id } => unlock(filepath, file_id),
        Commands::Logout => logout(),
    }
}

// -----------------------------------------------------

fn login(username: &String) {
    let mut stream = network::connect().expect("Error connecting to server");

    // Get salt
    network::write(
        &mut stream,
        ClientMessage::GetSalt {
            username: username.clone(),
        },
    )
    .expect("Error sending salt request to server");
    let option = match network::read(&mut stream).expect("Error reading response from server") {
        ServerMessage::GetSaltResponse { salt } => Some(salt),
        _ => None,
    };
    if option.is_none() {
        // Return an error in the future
        // Execution stops here
        todo!("Handle error: salt not found");
    }
    let salt: [u8; 32] = option.unwrap();

    // Prompt for password
    let password = rpassword::prompt_password("Your password: ")
        .unwrap()
        .to_string();

    // Generate root keys
    let mut master_key: [u8; 32] = [0; 32];
    hash_password(&mut master_key, &password, &salt);

    // Derive auth key from master key
    let auth_context: &'static str = "Authentication";
    let mut auth_key: [u8; 32] = [0; 32];
    derive_key(&master_key, &mut auth_key, auth_context);

    // Send to server
    network::write(
        &mut stream,
        ClientMessage::GetCredentials {
            username: username.clone(),
            auth_key,
        },
    )
    .expect("Error sending credential request to server");
    let option = match network::read(&mut stream).expect("Woopsies") {
        ServerMessage::GetCredentialsResponse {
            public_key,
            encrypted_private_key,
            nonce,
        } => Some((public_key, encrypted_private_key, nonce)),
        _ => todo!("Handle error: credentials not found"),
    };
    if option.is_none() {
        // Return an error in the future
        // Execution stops here
        todo!("Handle error: credentials not found");
    }
    let (public_key, encrypted_private_key, nonce) = option.unwrap();

    // Derive auth key from master key
    let auth_context: &'static str = "Authentication";
    let mut auth_key: [u8; 32] = [0; 32];
    derive_key(&master_key, &mut auth_key, auth_context);

    // Decrypt private key
    let mut decrypted_private_key: [u8; 32] = [0; 32];
}

fn logout() {}

fn signup(username: &String, password: &String) {}

fn send(filepath: &String, recipient_username: &String) {}

fn list_users() {}

fn list_messages() {}

fn reset() {
    let password = rpassword::prompt_password("New password: ").unwrap();
}

fn download(filepath: &String, file_id: &String) {}

fn unlock(filepath: &String, file_id: &String) {}

// -----------------------------------------------------

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
