use std::{
    io::{Write, Read, ErrorKind},
    marker::Sized,
    path::PathBuf,
    thread,
    time,
};

use bytes::BytesMut;

use utils;
use models::message::{Message, OpCode};
use error::{Error, Result};


/// Wait for a non-blocking connection until it's complete.
macro_rules! try_until_done {
    [ $e:expr ] => {
        loop {
            match $e {
                Ok(_) => break,
                Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                Err(why) => return Err(why),
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }
}


pub trait Connection: Sized {
    type Socket: Write + Read;

    fn socket(&mut self) -> &mut Self::Socket;

    fn ipc_path() -> PathBuf;

    fn connect() -> Result<Self>;

    fn disconnect(&self) -> Result<()>;

    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }

    fn handshake(&mut self, client_id: u64) -> Result<()> {
        let hs = json![{
            "client_id": client_id.to_string(),
            "v": 1,
            "nonce": utils::nonce()
        }];

        try_until_done!(self.send(Message::new(OpCode::Handshake, hs.clone())));
        try_until_done!(self.recv());

        Ok(())
    }

    fn ping(&mut self) -> Result<OpCode> {
        let message = Message::new(OpCode::Ping, json![{}]);
        self.send(message)?;
        let response = self.recv()?;
        Ok(response.opcode)
    }

    fn send(&mut self, message: Message) -> Result<()> {
        match message.encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket().write_all(bytes.as_ref())?;
            }
        };
        debug!("-> {:?}", message);
        Ok(())
    }

    fn recv(&mut self) -> Result<Message> {
        let mut buf = BytesMut::new();
        buf.resize(1024, 0);
        let n = self.socket().read(&mut buf)?;
        debug!("Received {} bytes", n);

        if n == 0 {
            return Err(Error::ConnectionClosed);
        }

        buf = buf.split_to(n);
        let message = Message::decode(&buf)?;
        debug!("<- {:?}", message);

        Ok(message)
    }
}
