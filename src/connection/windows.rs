use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read},
    os::windows::fs::OpenOptionsExt,
    path::PathBuf,
};

use super::base::Connection;
use crate::{DiscordError, Result};

pub struct Socket {
    socket: File,
}

impl Connection for Socket {
    type Socket = File;

    fn connect() -> Result<Self> {
        let path = Self::socket_path(0);

        let socket = OpenOptions::new().access_mode(0x3).open(&path)?;

        Ok(Self { socket })
    }

    fn ipc_path() -> PathBuf {
        PathBuf::from(r"\\.\pipe\")
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }

    fn try_read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.socket().metadata()?.len() == 0 {
            return Err(DiscordError::IoError(std::io::Error::new(
                ErrorKind::WouldBlock,
                "No data available",
            )));
        }

        Ok(self.socket().read(buf)?)
    }
}
