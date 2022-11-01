use std::{env, net::Shutdown, ops::RangeInclusive, path::PathBuf, time};

use super::base::Connection;
use crate::{DiscordError, Result};

use websocket::stream::sync::TcpStream;

pub struct UnixConnection {
    socket: TcpStream,
}

// The TCP port range that discord uses
const DISCORD_PORT_RANGE: RangeInclusive<u16> = 6463..=6472;

impl Connection for UnixConnection {
    type Socket = TcpStream;

    fn connect() -> Result<Self> {
        let mut tcp_stream = None;

        for i in DISCORD_PORT_RANGE {
            match TcpStream::connect(("127.0.0.1", i)) {
                Ok(v) => tcp_stream = Some(v),
                Err(_) => continue,
            };
        }

        if let Some(socket) = tcp_stream {
            socket.set_nonblocking(true)?;
            socket.set_write_timeout(Some(time::Duration::from_secs(30)))?;
            socket.set_read_timeout(Some(time::Duration::from_secs(30)))?;

            Ok(Self { socket })
        } else {
            Err(DiscordError::MissingSocket)
        }
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

impl Drop for UnixConnection {
    fn drop(&mut self) {
        if self.socket.shutdown(Shutdown::Both).is_err() {
            error!("Failed to properly shut down socket");
        }
    }
}
