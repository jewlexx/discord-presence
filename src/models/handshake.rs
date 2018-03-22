use uuid::Uuid;
use super::Payload;

#[derive(Debug, Default, Serialize)]
pub struct Handshake {
    nonce: String,
    v: u32,
    client_id: String,
}

impl Handshake {
    pub fn new(client_id: u64, version: u32) -> Self {
        Self {
            nonce: Uuid::new_v4().to_string(),
            v: version,
            client_id: client_id.to_string()
        }
    }
}

impl Payload for Handshake {}
