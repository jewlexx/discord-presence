use std::path::{Path, PathBuf};

use num_traits::FromPrimitive;
use quork::prelude::ListVariants;

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
        SocketLocation::from_u8(self.0 & 0b1111_0000)
    }

    pub const fn get_number(self) -> u8 {
        self.0 & 0b0000_1111
    }

    pub fn validate(self) -> bool {
        self.get_number() < 10 && self.try_get_location().is_some()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, num_derive::FromPrimitive, ListVariants)]
#[repr(u8)]
pub enum SocketLocation {
    Root = 0b0000_0000,
    Flatpak = 0b1000_0000,
    Snap = 0b0100_0000,
    SnapCanary = 0b1100_0000,
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
        }

        for location in Self::VARIANTS {
            let path = location
                .append_path(ipc_path.as_ref())
                .join(socket_path.as_ref());

            if path.exists() {
                return Some(path);
            }
        }

        None
    }
}
