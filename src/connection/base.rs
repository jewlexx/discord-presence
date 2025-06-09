use crate::{
    error::{DiscordError, Result},
    models::message::{FrameHeader, Message, OpCode, MAX_RPC_FRAME_SIZE},
    utils,
};
use bytes::BytesMut;
use quork::prelude::ListVariants;
use serde_json::json;
use std::{
    io::{Read, Write},
    marker::Sized,
    path::{Path, PathBuf},
    thread,
    time::{self, Duration},
};

/// Wait for a non-blocking connection until it's complete.
fn try_until_done<T>(result: Result<T>) -> Result<T> {
    loop {
        match result {
            Ok(v) => return Ok(v),
            Err(why) if !why.io_would_block() => return Err(why),
            _ => {}
        }

        thread::sleep(time::Duration::from_micros(500));
    }
}

#[derive(Debug, Copy, Clone, ListVariants)]
enum SocketLocation {
    Root,
    Flatpak,
    Snap,
    SnapCanary,
}

impl SocketLocation {
    pub fn append_path(self, ipc_path: impl AsRef<Path>) -> PathBuf {
        match self {
            SocketLocation::Root => ipc_path.as_ref().to_owned(),
            SocketLocation::Flatpak => ipc_path.as_ref().join("app").join("com.discordapp.Discord"),
            SocketLocation::Snap => ipc_path.as_ref().join("snap.discord"),
            SocketLocation::SnapCanary => ipc_path.as_ref().join("snap.discord-canary"),
        }
    }

    pub fn test_paths(
        ipc_path: impl AsRef<Path>,
        socket_path: impl AsRef<Path>,
    ) -> Option<PathBuf> {
        if cfg!(windows) {
            let path = Self::Root.append_path(ipc_path).join(socket_path);
            return path.exists().then_some(path);
        } else {
            for location in Self::VARIANTS {
                let path = location
                    .append_path(ipc_path.as_ref())
                    .join(socket_path.as_ref());

                if path.exists() {
                    return Some(path);
                }
            }
        }

        None
    }
}

pub trait Connection: Sized {
    type Socket: Write + Read;

    /// Time for socket read/write operations
    /// 1 second higher than Discord's rate limit timeout of 15 seconds
    const READ_WRITE_TIMEOUT: Duration = Duration::from_secs(16);

    /// The internally stored socket connection.
    fn socket(&mut self) -> &mut Self::Socket;

    /// The base path were the socket is located.
    fn ipc_path() -> PathBuf;

    /// Establish a new connection to the server.
    fn connect() -> Result<Self>;

    /// The full socket path.
    fn socket_path(n: u8) -> PathBuf {
        let socket_path = format!("discord-ipc-{n}");
        let ipc_path = Self::ipc_path();

        SocketLocation::test_paths(&ipc_path, &socket_path).unwrap_or(ipc_path.join(socket_path))
    }

    /// Perform a handshake on this socket connection.
    /// Will block until complete.
    fn handshake(&mut self, client_id: u64) -> Result<Message> {
        let hs = json![{
            "client_id": client_id.to_string(),
            "v": 1,
            "nonce": utils::nonce()
        }];

        let msg = Message::new(OpCode::Handshake, hs)?;
        try_until_done(self.send(&msg))?;
        let msg = try_until_done(self.recv())?;

        Ok(msg)
    }

    /// Ping the server and get a pong response.
    /// Will block until complete.
    fn ping(&mut self) -> Result<OpCode> {
        let message = Message::new(OpCode::Ping, json![{}])?;
        try_until_done(self.send(&message))?;
        let response = try_until_done(self.recv())?;
        Ok(response.opcode)
    }

    /// Send a message to the server.
    fn send(&mut self, message: &Message) -> Result<()> {
        match message.encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                assert!(bytes.len() <= MAX_RPC_FRAME_SIZE);
                self.socket().write_all(&bytes)?;
            }
        };
        trace!("-> {:?}", message);
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
        trace!("Received {} bytes for payload", n);

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
