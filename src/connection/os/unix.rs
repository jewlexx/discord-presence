use num_traits::FromPrimitive;

use crate::{connection::base::Connection, Result};
use std::{env, net::Shutdown, os::unix::net::UnixStream, path::PathBuf};

pub struct Socket {
    socket: UnixStream,
}

impl Connection for Socket {
    type Socket = UnixStream;

    fn connect() -> Result<Self> {
        let connection_name = Self::socket_path(0);
        let socket = UnixStream::connect(connection_name)?;
        socket.set_read_timeout(Some(Self::READ_WRITE_TIMEOUT))?;
        socket.set_write_timeout(Some(Self::READ_WRITE_TIMEOUT))?;
        Ok(Self { socket })
    }

    fn ipc_path() -> PathBuf {
        let tmp = env::var("XDG_RUNTIME_DIR")
            .or_else(|_| env::var("TMPDIR"))
            .or_else(|_| match env::temp_dir().to_str() {
                None => Err("Failed to convert temp_dir"),
                Some(tmp) => Ok(tmp.to_owned()),
            })
            .unwrap_or_else(|_| "/tmp".to_owned());
        PathBuf::from(tmp)
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, num_derive::FromPrimitive)]
#[repr(u8)]
pub enum SocketLocation {
    Root = 0b0000_0000,
    Flatpak = 0b1000_0000,
    Snap = 0b0100_0000,
    SnapCanary = 0b1100_0000,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SocketId(u8);

impl SocketId {
    pub const fn new(number: u8, location: SocketLocation) -> Self {
        let loc = location as u8;
        Self(number | loc)
    }

    pub fn get_location(self) -> SocketLocation {
        unsafe { self.try_get_location().unwrap_unchecked() }
    }

    pub fn try_get_location(self) -> Option<SocketLocation> {
        SocketLocation::from_u8(self.0 & 0b0000_1111)
    }

    pub const fn get_number(self) -> u8 {
        self.0 & 0b0000_1111
    }

    pub fn validate(self) -> bool {
        self.get_number() < 10 && self.try_get_location().is_some()
    }
}

// TODO: Add function for connecting to known socket path type
// Use struct with bits

impl Drop for Socket {
    fn drop(&mut self) {
        if self.socket.shutdown(Shutdown::Both).is_err() {
            error!("Failed to properly shut down socket");
        }
    }
}
