use std::path::{Path, PathBuf};

use num_traits::FromPrimitive;
use quork::prelude::ListVariants;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SocketId(u8);

impl SocketId {
    pub const UNKNOWN_NUMBER: u8 = 10;

    pub const fn new(number: u8, location: SocketLocation) -> Self {
        let loc = location as u8;
        Self::new_bytes(number, loc)
    }

    const fn new_bytes(number: u8, location: u8) -> Self {
        Self(number | location)
    }

    pub const fn without_location(number: u8) -> Self {
        Self(number)
    }

    pub const fn without_number(location: SocketLocation) -> Self {
        Self(location as u8)
    }

    pub const fn blank() -> Self {
        Self(0)
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

    pub fn resolve_path(self, ipc_root: impl AsRef<Path>) -> Option<PathBuf> {
        fn find_path_unchecked(id: SocketId, ipc_root: impl AsRef<Path>) -> Option<PathBuf> {
            assert!(id.validate(), "Invalid SocketId: {id:?}");

            let i = id.get_number();
            let socket_path = format!("discord-ipc-{i}");
            if let Some(location) = id.try_get_location() {
                let path = location.append_to_root(ipc_root).join(socket_path);
                let path_opt = path.exists().then_some(path);

                if path_opt.is_some() {
                    return path_opt;
                }
            } else if let Some(path) = SocketLocation::find_path(ipc_root.as_ref(), socket_path) {
                return Some(path);
            }

            None
        }

        if self.get_number() == Self::UNKNOWN_NUMBER {
            let loc_bytes = self.try_get_location().map_or(0, |loc| loc as u8);
            for i in 0..10 {
                let id = Self::new_bytes(i, loc_bytes);
                if let Some(path) = find_path_unchecked(id, &ipc_root) {
                    return Some(path);
                }
            }

            None
        } else {
            find_path_unchecked(self, ipc_root)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, num_derive::FromPrimitive, ListVariants)]
#[repr(u8)]
pub enum SocketLocation {
    Root = 0b1000_0000,
    Flatpak = 0b0100_0000,
    Snap = 0b1100_0000,
    SnapCanary = 0b0010_0000,
}

impl SocketLocation {
    pub fn append_to_root(self, ipc_root: impl AsRef<Path>) -> PathBuf {
        match self {
            SocketLocation::Root => ipc_root.as_ref().to_owned(),
            SocketLocation::Flatpak => ipc_root.as_ref().join("app").join("com.discordapp.Discord"),
            SocketLocation::Snap => ipc_root.as_ref().join("snap.discord"),
            SocketLocation::SnapCanary => ipc_root.as_ref().join("snap.discord-canary"),
        }
    }

    pub fn find_path(ipc_root: impl AsRef<Path>, socket_path: impl AsRef<Path>) -> Option<PathBuf> {
        #[cfg(windows)]
        {
            let path = Self::Root.append_to_root(ipc_root).join(socket_path);
            return path.exists().then_some(path);
        }

        for location in Self::VARIANTS {
            let path = location
                .append_to_root(ipc_root.as_ref())
                .join(socket_path.as_ref());

            if path.exists() {
                return Some(path);
            }
        }

        None
    }
}
