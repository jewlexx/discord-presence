use std::{
    time,
    path::PathBuf,
    env,
    os::unix::net::UnixStream,
    net::Shutdown,
};

use super::base::Connection;
use error::Result;


pub struct UnixConnection {
    socket: UnixStream,
}

impl Connection for UnixConnection {
    type Socket = UnixStream;

    fn connect() -> Result<Self> {
        let connection_name = Self::socket_path(0);
        let socket = UnixStream::connect(connection_name)?;
        socket.set_nonblocking(true)?;
        socket.set_write_timeout(Some(time::Duration::from_secs(30)))?;
        socket.set_read_timeout(Some(time::Duration::from_secs(30)))?;
        Ok(Self { socket })
    }

    fn ipc_path() -> PathBuf {
        let tmp = env::var("XDG_RUNTIME_DIR")
            .or_else(|_| env::var("TMPDIR"))
            .or_else(|_| {
                match env::temp_dir().to_str() {
                    None => Err("Failed to convert temp_dir"),
                    Some(tmp) => Ok(tmp.to_string())
                }
            })
            .unwrap_or("/tmp".to_string());
        PathBuf::from(tmp)
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }
}

impl Drop for UnixConnection {
    fn drop(&mut self) {
        self.socket.shutdown(Shutdown::Both)
            .expect("Failed to properly shut down socket");
    }
}
