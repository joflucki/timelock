use crate::network;
use tabled::settings::Style;
use tabled::*;

pub fn list_users() {
    let mut stream = network::connect().expect("Failed to connect to server");
    let message = shared::messages::ClientMessage::ListUsers {};
    network::write(&mut stream, message).expect("Failed to send request");

    let users = match network::read(&mut stream).expect("Failed to read response from server") {
        shared::messages::ServerMessage::ListUsersResponse { users } => Some(users),
        _ => todo!(),
    }
    .expect("Failed to parse response from server");

    let mut table = Table::new(users);
    table.with(Style::modern_rounded());
    println!("{}", table);
}
