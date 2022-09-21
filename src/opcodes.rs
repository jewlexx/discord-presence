#![allow(missing_docs)]

// Opcodes used by the discord client
pub enum OPCODES {
  /// This is to connect to discord
  Handshake,

  /// When sending data as to the ipc
  Frame,

  // Socket closed by discord
  Close,

  // ping event
  Ping,

  // pong event
  Pong,
}
