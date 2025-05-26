use crate::crypto::*;
use crate::network;
use crate::utils;
use shared::crypto::*;
use tabled::settings::Style;
use tabled::*;

pub fn list_messages() {
    let username = utils::load_username().expect("Failed to load username");
    let (_, _, _, private_key, _, server_public_key) =
        utils::load_keys().expect("Failed to load keys");

    let mut shared_key: [u8; KEY_SIZE] = [0; KEY_SIZE];
    exchange_keys(&server_public_key, &private_key, &mut shared_key);

    let mut mac: [u8; MAC_SIZE] = [0; MAC_SIZE];
    authenticate(&mut mac, &shared_key, username.as_bytes());

    let mut stream = network::connect().expect("Failed to connect to server");
    let message = shared::messages::ClientMessage::ListMessages {
        username: username.clone(),
        mac,
    };

    network::write(&mut stream, message).expect("Failed to send request");

    let messages = match network::read(&mut stream).expect("Failed to read response from server") {
        shared::messages::ServerMessage::ListMessagesResponse { messages } => Some(messages),
        _ => todo!(),
    }
    .expect("Failed to parse response from server");

    let mut table = Table::new(messages);
    table.with(Style::modern_rounded());
    println!("{}", table);
}
