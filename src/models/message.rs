use std::io::{self, Write, Read};
use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};
use serde_json;
use serde::Serialize;
use error::{Result, Error};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    Handshake,
    Frame,
    Close,
    Ping,
    Pong,
}

// FIXME: Use TryFrom trait when stable
impl OpCode {
    fn try_from(int: u32) -> Result<Self> {
        match int {
            0 => Ok(OpCode::Handshake),
            1 => Ok(OpCode::Frame),
            2 => Ok(OpCode::Close),
            3 => Ok(OpCode::Ping),
            4 => Ok(OpCode::Pong),
            _ => Err(Error::Conversion)
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
        let opcode = OpCode::try_from(reader.read_u32::<LittleEndian>()?)?;
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

    #[test]
    fn test_opcode() {
        assert_eq!(OpCode::try_from(0).ok(), Some(OpCode::Handshake));
        assert_eq!(OpCode::try_from(4).ok(), Some(OpCode::Pong));
        assert_eq!(OpCode::try_from(5).ok(), None);
    }
}
