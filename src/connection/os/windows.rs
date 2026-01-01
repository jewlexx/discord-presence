use std::{
    fs::{File, OpenOptions},
    io::{self, Read},
    os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
    path::PathBuf,
    ptr,
};

use windows_sys::Win32::{
    Foundation::{GetLastError, ERROR_BROKEN_PIPE},
    System::Pipes::PeekNamedPipe,
};

use crate::{
    connection::{base::Connection, location::SocketId},
    DiscordError, Result,
};

/// Socket connection for Windows systems.
pub struct Socket {
    socket: File,
}

impl Connection for Socket {
    type Socket = File;

    fn connect_with_id(id: SocketId) -> Result<Self> {
        let path = Self::socket_path(id);

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
        // peek the pipe to see if there is data available
        let mut bytes_available: u32 = 0;
        let ok = unsafe {
            PeekNamedPipe(
                self.socket().as_raw_handle(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                &raw mut bytes_available,
                ptr::null_mut(),
            )
        } != 0;

        // an error occurred while peeking the pipe
        if !ok {
            let err = unsafe { GetLastError() };

            // if the pipe is broken, treat it as EOF
            if err == ERROR_BROKEN_PIPE {
                return Ok(0);
            }

            #[allow(clippy::cast_possible_wrap)]
            return Err(DiscordError::IoError(io::Error::from_raw_os_error(
                err as i32,
            )));
        }

        if bytes_available == 0 {
            return Err(DiscordError::IoError(io::Error::new(
                io::ErrorKind::WouldBlock,
                "No data available",
            )));
        }

        Ok(self.socket().read(buf)?)
    }
}
