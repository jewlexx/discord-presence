use std::{ops::RangeInclusive, path::PathBuf};

use super::base::Connection;
use crate::{DiscordError, Result};

use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;
use websocket::stream::sync::TcpStream;

#[derive(Debug)]
pub struct RWSocketConnection {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl std::io::Read for RWSocketConnection {
    fn read(&mut self, mut buf: &mut [u8]) -> std::io::Result<usize> {
        use std::io::{Error, ErrorKind, Write};
        match self.socket.read_message() {
            Ok(v) => {
                let mut data = v.into_data();
                buf.write_all(&mut data)?;
                Ok(data.len())
            }
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        }
    }
}

impl std::io::Write for RWSocketConnection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        use std::io::{Error, ErrorKind};
        match self
            .socket
            .write_message(tungstenite::Message::Binary(buf.to_vec()))
        {
            Ok(_) => Ok(buf.len()),
            Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SocketConnection {
    pub socket: RWSocketConnection,
}

// The TCP port range that discord uses
const DISCORD_PORT_RANGE: RangeInclusive<u16> = 6463..=6472;

impl Connection for SocketConnection {
    type Socket = RWSocketConnection;

    fn connect(client_id: u64) -> Result<Self> {
        let tcp_stream = if let Ok(port) = std::env::var("DISCORD_PORT") {
            let url_raw = format!("ws://127.0.0.1:{port}/?v=1&client_id={client_id}");
            let url = Url::parse(&url_raw).expect("Invalid url");
            match connect(url) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        } else {
            let mut valid_tcp = None;
            // TODO: Try pinging all at once and then only return the one that succeeds
            for i in DISCORD_PORT_RANGE {
                let url_raw = format!("ws://127.0.0.1:{i}/?v=1&client_id={client_id}");
                let url = Url::parse(&url_raw).expect("Invalid url");
                match connect(url) {
                    Ok(v) => valid_tcp = Some(v),
                    Err(_) => continue,
                };
            }
            valid_tcp
        };

        if let Some((socket, _)) = tcp_stream {
            Ok(Self {
                socket: RWSocketConnection { socket },
            })
        } else {
            Err(DiscordError::MissingSocket)
        }
    }

    fn ipc_path() -> PathBuf {
        PathBuf::new()
    }

    fn socket(&mut self) -> &mut Self::Socket {
        &mut self.socket
    }
}

impl Drop for SocketConnection {
    fn drop(&mut self) {
        if self.socket.socket.close(None).is_err() {
            error!("Failed to properly shut down socket");
        }
    }
}
