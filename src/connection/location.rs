//! Socket location and identifier handling for Discord IPC.

use std::path::{Path, PathBuf};

use num_traits::FromPrimitive;
use quork::prelude::ListVariants;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(transparent)]
/// Represents a Discord IPC socket identifier.
/// The identifier consists of a number (0-9) and a location (e.g., root, flatpak, snap).
pub struct SocketId(u8);

impl SocketId {
    /// An identifier for an unknown socket number.
    pub const UNKNOWN_NUMBER: u8 = 0b0000_1111;

    #[must_use]
    /// Creates a new `SocketId` with the specified number and location.
    pub const fn new(number: u8, location: SocketLocation) -> Self {
        let loc = location as u8;
        Self::new_bytes(number, loc)
    }

    const fn new_bytes(number: u8, location: u8) -> Self {
        Self(number | location)
    }

    #[must_use]
    /// Creates a new `SocketId` without a specific location.
    pub const fn without_location(number: u8) -> Self {
        Self(number)
    }

    #[must_use]
    /// Creates a new `SocketId` without a specific number.
    pub const fn without_number(location: SocketLocation) -> Self {
        Self(location as u8)
    }

    #[must_use]
    /// Creates a `SocketId` that does not specify a location or number.
    pub const fn blank() -> Self {
        Self::without_location(Self::UNKNOWN_NUMBER)
    }

    #[must_use]
    /// Gets the location of the socket identifier.
    pub fn get_location(self) -> Option<SocketLocation> {
        SocketLocation::from_u8(self.0 & 0b1111_0000)
    }

    #[must_use]
    /// Gets the number of the socket identifier.
    pub const fn get_number(self) -> u8 {
        self.0 & 0b0000_1111
    }

    #[must_use]
    /// Checks if the socket identifier is valid.
    ///
    /// Note this does not check if the socket actually exists on the filesystem.
    pub fn validate(self) -> bool {
        self.validate_number() && self.validate_location()
    }

    #[must_use]
    /// Checks if the socket identifier has a valid number.
    pub fn validate_number(self) -> bool {
        self.get_number() < 10
    }

    #[must_use]
    /// Checks if the socket identifier has a valid location.
    pub fn validate_location(self) -> bool {
        self.get_location().is_some()
    }

    #[must_use]
    /// Attempts to resolve the socket identifier to a path based on the IPC root directory.
    ///
    /// If the number is unknown, it will try to find a valid socket path by iterating
    /// through possible numbers and locations.
    ///
    /// If the location is not specified, it will search in all known locations.
    pub fn resolve_path(self, ipc_root: impl AsRef<Path>) -> Option<PathBuf> {
        fn find_path_unchecked(id: SocketId, ipc_root: impl AsRef<Path>) -> Option<PathBuf> {
            assert!(id.validate_number(), "Invalid SocketId: {id:?}");

            let i = id.get_number();
            let socket_path = format!("discord-ipc-{i}");
            if let Some(location) = id.get_location() {
                assert!(id.validate_location(), "Invalid SocketId: {id:?}");

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

        if self.get_number() >= 10 {
            let loc_bytes = self.get_location().map_or(0, |loc| loc as u8);
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
/// Represents the location of a Discord IPC socket.
pub enum SocketLocation {
    /// The root location. This is used for system installed Discord instances.
    Root = 0b1000_0000,
    /// The Flatpak location. This is used for Discord installed via Flatpak.
    Flatpak = 0b0100_0000,
    /// The Snap location. This is used for Discord installed via Snap.
    Snap = 0b1100_0000,
    /// The Snap Canary location. This is used for Discord Canary installed via Snap.
    SnapCanary = 0b0010_0000,
}

impl SocketLocation {
    /// Append the socket location to the given IPC root path.
    pub fn append_to_root(self, ipc_root: impl AsRef<Path>) -> PathBuf {
        match self {
            SocketLocation::Root => ipc_root.as_ref().to_owned(),
            SocketLocation::Flatpak => ipc_root.as_ref().join("app").join("com.discordapp.Discord"),
            SocketLocation::Snap => ipc_root.as_ref().join("snap.discord"),
            SocketLocation::SnapCanary => ipc_root.as_ref().join("snap.discord-canary"),
        }
    }

    /// Find a valid location for the given IPC root and socket path.
    pub fn find_path(ipc_root: impl AsRef<Path>, socket_path: impl AsRef<Path>) -> Option<PathBuf> {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                let path = Self::Root.append_to_root(ipc_root).join(socket_path);
                path.exists().then_some(path)
            } else {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_numbers() {
        for i in 0..10 {
            let id = SocketId::new(i, SocketLocation::Root);
            assert!(id.validate(), "SocketId {i} should be valid");
            assert_eq!(id.get_number(), i, "SocketId {i} should have number {i}");
        }
    }

    #[test]
    fn test_socket_locations() {
        let root_id = SocketId::new(0, SocketLocation::Root);
        assert_eq!(root_id.get_location(), Some(SocketLocation::Root));

        let flatpak_id = SocketId::new(1, SocketLocation::Flatpak);
        assert_eq!(flatpak_id.get_location(), Some(SocketLocation::Flatpak));

        let snap_id = SocketId::new(2, SocketLocation::Snap);
        assert_eq!(snap_id.get_location(), Some(SocketLocation::Snap));

        let snap_canary_id = SocketId::new(3, SocketLocation::SnapCanary);
        assert_eq!(
            snap_canary_id.get_location(),
            Some(SocketLocation::SnapCanary)
        );
    }
}
