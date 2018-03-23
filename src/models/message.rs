use byte::{TryRead, TryWrite, BytesExt, Result};
use byte::ctx::{Endian, LE, Str};
use serde_json;
use serde::Serialize;

#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    Handshake,
    Frame,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct Message {
    opcode:  u32,
    message: String,
}

impl<'a> TryRead<'a, Endian> for Message {
    fn try_read(bytes: &'a [u8], endian: Endian) -> Result<(Self, usize)> {
        let offset = &mut 0;
        let opcode: u32 = bytes.read_with(offset, endian)?;
        let message_length = bytes.read_with::<u32>(offset, endian)? as usize;
        let message = bytes.read_with::<&str>(offset, Str::Len(message_length))?.to_string();
        Ok((Message { opcode, message }, *offset))
    }
}

impl TryWrite<Endian> for Message {
    fn try_write(self, bytes: &mut [u8], endian: Endian) -> Result<usize> {
        let offset = &mut 0;
        bytes.write_with::<u32>(offset, self.opcode, endian)?;
        bytes.write_with::<u32>(offset, self.message.len() as u32, endian)?;
        bytes.write_with::<&str>(offset, self.message.as_ref(), ())?;
        Ok(*offset)
    }
}

impl Message {
    pub fn new<T>(opcode: OpCode, message: T) -> Self
        where T: Serialize
    {
        Message {
            opcode: opcode as u32,
            message: serde_json::to_string(&message).unwrap()
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![0; 2*4+self.message.len()];
        bytes.write_with(&mut 0, self.clone(), LE)?;
        bytes.shrink_to_fit();
        Ok(bytes)
    }

    #[allow(dead_code)]
    pub fn decode<'a>(bytes: &'a [u8]) -> Result<Self> {
        let message: Message = bytes.read_with(&mut 0, LE)?;
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Something {
        empty: bool
    }

    #[test]
    fn test_encoder() {
        let msg = Message::new(OpCode::Frame, Something { empty: true });
        let encoded = msg.encode().unwrap();
        let decoded = Message::decode(encoded.as_ref()).unwrap();
        assert_eq!(msg, decoded);
    }
}
