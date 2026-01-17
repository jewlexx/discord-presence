use std::{
    io::{Read, Write},
    marker::Sized,
    path::PathBuf,
    thread,
    time::Duration,
};

use bytes::BytesMut;
use serde_json::json;

use crate::{
    connection::location::SocketId,
    error::{DiscordError, Result},
    models::message::{FrameHeader, Message, OpCode, MAX_RPC_FRAME_SIZE},
};

/// Wait for a non-blocking connection until it's complete.
fn try_until_done<T, F>(mut operation: F) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    loop {
        match operation() {
            Ok(v) => return Ok(v),
            Err(why) if !why.io_would_block() => return Err(why),
            _ => {}
        }

        thread::sleep(Duration::from_micros(500));
    }
}

pub trait Connection: Sized {
    type Socket: Write + Read;

    /// Time for socket read/write operations
    /// 1 second higher than Discord's rate limit timeout of 15 seconds
    #[cfg(unix)]
    const READ_WRITE_TIMEOUT: Duration = Duration::from_secs(16);

    /// The internally stored socket connection.
    fn socket(&mut self) -> &mut Self::Socket;

    /// The base path were the socket is located.
    fn ipc_path() -> PathBuf;

    /// Establish a new connection to the server, without specifying a location.
    fn connect() -> Result<Self> {
        Self::connect_with_id(SocketId::blank())
    }

    fn connect_with_id(id: SocketId) -> Result<Self>;

    /// The full socket path.
    fn socket_path(id: SocketId) -> PathBuf {
        let ipc_root = Self::ipc_path();

        id.resolve_path(&ipc_root)
            .unwrap_or_else(|| ipc_root.join(format!("discord-ipc-{}", id.get_number())))
    }

    /// Perform a handshake on this socket connection.
    /// Will block until complete.
    fn handshake(&mut self, client_id: u64) -> Result<Message> {
        let hs = json![{
            "client_id": client_id.to_string(),
            "v": 1,
            "nonce": crate::nonce()
        }];

        let msg = Message::new(OpCode::Handshake, hs)?;

        try_until_done(|| self.send(&msg))?;

        let msg = try_until_done(|| self.recv())?;

        Ok(msg)
    }

    /// Send a message to the server.
    fn send(&mut self, message: &Message) -> Result<()> {
        match message.encode() {
            Err(why) => error!("{why:?}"),
            Ok(bytes) => {
                assert!(bytes.len() <= MAX_RPC_FRAME_SIZE);
                self.socket().write_all(&bytes)?;
            }
        }

        trace!("-> {message:?}");
        Ok(())
    }

    /// Receive a message from the server.
    fn recv(&mut self) -> Result<Message> {
        // Read header
        let mut buf = BytesMut::new();
        buf.resize(std::mem::size_of::<FrameHeader>(), 0);

        trace!("Reading header");
        let n = self.try_read(&mut buf)?;
        trace!("Received {n} bytes for header");

        if n == 0 {
            return Err(DiscordError::ConnectionClosed);
        }

        if n != std::mem::size_of::<FrameHeader>() {
            return Err(DiscordError::HeaderLength);
        }

        // SAFETY: the length of buf is already checked that the header is the correct size
        let header = unsafe { FrameHeader::from_bytes(buf.as_ref()).unwrap_unchecked() };

        let mut message_buf = BytesMut::new();
        message_buf.resize(header.message_length(), 0);

        trace!("Reading payload");
        let n = self.try_read(&mut message_buf)?;
        trace!("Received {n} bytes for payload");

        if n == 0 {
            return Err(DiscordError::NoMessage);
        }

        let mut payload = String::with_capacity(header.message_length());
        message_buf.as_ref().read_to_string(&mut payload)?;
        trace!("<- {:?} = {:?}", header.opcode(), payload);

        Ok(Message {
            opcode: header.opcode(),
            payload,
        })
    }

    fn try_read(&mut self, buf: &mut [u8]) -> Result<usize> {
        Ok(self.socket().read(buf)?)
    }
}
