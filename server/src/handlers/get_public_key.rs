use anyhow::Result;
use native_tls::TlsStream;
use std::net::TcpStream;

use crate::{network, utils};

pub fn get_public_key(stream: &mut TlsStream<TcpStream>, username: &str) -> Result<()> {
    let (_, _, public_key, _, _) = utils::load_credentials(username)?;
    network::write(
        stream,
        shared::frames::ServerFrame::GetPublicKeyResponse {
            public_key: public_key,
        },
    )?;
    Ok(())
}
