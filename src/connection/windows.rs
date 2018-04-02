extern crate named_pipe;

use std::{
    io::{Write, Read},
    time,
    path::PathBuf,
    fmt::Debug
};

use super::base::Connection;
use models::{Payload, Message, OpCode};
use error::Result;

use self::named_pipe::PipeClient;

pub struct WindowsConnection {
    socket: PipeClient,
}

impl WindowsConnection {
    fn ipc_path() -> PathBuf {
        PathBuf::from(r"\\.\pipe\")
    }

    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }
}

impl Connection for WindowsConnection {
    fn connect() -> Result<Self> {
        let connection_name = Self::socket_path(0);
        let mut socket = PipeClient::connect(connection_name)?;
        socket.set_write_timeout(Some(time::Duration::from_secs(30)));
        socket.set_read_timeout(Some(time::Duration::from_secs(30)));
        Ok(Self { socket })
    }

    fn send<T>(&mut self, opcode: OpCode, payload: T) -> Result<()>
        where T: Payload + Debug
    {
        debug!("payload: {:#?}", payload);
        match Message::new(opcode, payload).encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket.write_all(bytes.as_ref())?;
                debug!("sent opcode: {:?}", opcode);
                self.recv()?;
            }
        };
        Ok(())
    }

    fn recv(&mut self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let n = self.socket.read(buf.as_mut_slice())?;
        buf.resize(n, 0);
        debug!("{:?}", Message::decode(&buf));
        Ok(buf)
    }
}
