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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SocketLocation {
    Root,
    Flatpak,
    Snap,
    SnapCanary,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SocketId(u8);

impl SocketId {
    pub const ROOT: Self = Self(0b0000_0000);
    pub const FLATPAK: Self = Self(0b1000_0000);
    pub const SNAP: Self = Self(0b0100_0000);
    pub const SNAP_CANARY: Self = Self(0b1100_0000);

    pub const fn new(number: u8, location: SocketLocation) -> Self {
        let loc = match location {
            SocketLocation::Root => Self::ROOT,
            SocketLocation::Flatpak => Self::FLATPAK,
            SocketLocation::Snap => Self::SNAP,
            SocketLocation::SnapCanary => Self::SNAP_CANARY,
        };
        Self(number | loc.0)
    }

    pub const fn get_location(self) -> SocketLocation {
        unsafe { self.try_get_location().unwrap_unchecked() }
    }

    pub const fn try_get_location(self) -> Option<SocketLocation> {
        match Self(self.0 & 0b0000_1111).0 {
            0b0000_0000 => Some(SocketLocation::Root),
            0b1000_0000 => Some(SocketLocation::Flatpak),
            0b0100_0000 => Some(SocketLocation::Snap),
            0b1100_0000 => Some(SocketLocation::SnapCanary),
            _ => None,
        }
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
