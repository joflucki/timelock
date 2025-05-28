use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

use crate::{network, utils};

pub fn get_salt(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
    let (_, _, _, _, salt) = utils::load_credentials(username)?;
    network::write(
        stream,
        shared::frames::ServerFrame::GetSaltResponse { salt: salt },
    )?;
    Ok(())
}
