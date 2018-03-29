use super::Payload;
use utils::nonce;


#[derive(Debug, Default, Serialize)]
pub struct Handshake {
    nonce: String,
    v: u32,
    client_id: String,
}

impl Handshake {
    pub fn new(client_id: u64, version: u32) -> Self {
        Self {
            nonce: nonce(),
            v: version,
            client_id: client_id.to_string()
        }
    }
}

impl Payload for Handshake {}
