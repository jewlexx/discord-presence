use super::base::Connection;
use crate::Result;
use named_pipe::PipeClient;
use std::{path::PathBuf, time};

pub struct Socket {
    socket: PipeClient,
}

impl Connection for Socket {
    type Socket = PipeClient;

    fn connect() -> Result<Self> {
        let connection_name = Self::socket_path(0);
        let mut socket = PipeClient::connect(connection_name)?;
        // Discord rate limit timeout is 15 seconds, so 16 should account for that
        socket.set_write_timeout(Some(time::Duration::from_secs(16)));
        socket.set_read_timeout(Some(time::Duration::from_secs(16)));
        Ok(Self { socket })
    }

    fn ipc_path() -> PathBuf {
        PathBuf::from(r"\\.\pipe\")
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }
}
