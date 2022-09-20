#![allow(missing_docs)]

// Opcodes used by the discord client
pub enum OPCODES {
  /// This is to connect to discord
  Handshake,

  /// When sending data as to the ipc
  Frame,
  Close,
  Ping,
  Pong
}