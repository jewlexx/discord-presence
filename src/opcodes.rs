#![allow(missing_docs)]

// Opcodes used by the discord client
pub enum OPCODES {
  Handshake,
  Frame,
  Close,
  Ping,
  Pong
}