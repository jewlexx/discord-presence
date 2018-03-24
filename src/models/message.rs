use std::io::{self, Write, Read, Result};
use std::convert::From;
use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};
use serde_json;
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    Handshake,
    Frame,
    Invalid,
}

impl From<u32> for OpCode {
    fn from(int: u32) -> Self {
        match int {
            0 => OpCode::Handshake,
            1 => OpCode::Frame,
            _ => OpCode::Invalid,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    opcode:  OpCode,
    message: String,
}

impl Message {
    pub fn new<T>(opcode: OpCode, message: T) -> Self
        where T: Serialize
    {
        Message {
            opcode: opcode,
            message: serde_json::to_string(&message).unwrap()
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![];
        bytes.write_u32::<LittleEndian>(self.opcode as u32)?;
        bytes.write_u32::<LittleEndian>(self.message.len() as u32)?;
        write!(bytes, "{}", self.message)?;
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        let mut reader = io::Cursor::new(bytes);
        let mut message = String::new();
        let opcode = OpCode::from(reader.read_u32::<LittleEndian>()?);
        reader.read_u32::<LittleEndian>()?;
        reader.read_to_string(&mut message)?;
        Ok(Message { opcode, message })
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
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}
