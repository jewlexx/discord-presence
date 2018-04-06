use std::{
    io::{Write, Read},
    marker::Sized,
    path::PathBuf,
};

use models::message::Message;
use error::Result;


pub trait Connection: Sized {
    type Socket: Write + Read;


    fn socket(&mut self) -> &mut Self::Socket;

    fn ipc_path() -> PathBuf;

    fn connect() -> Result<Self>;

    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }

    fn send(&mut self, message: Message) -> Result<()> {
        debug!("{:?}", message);
        match message.encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket().write_all(bytes.as_ref())?;
            }
        };
        Ok(())
    }

    fn recv(&mut self) -> Result<Message> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let n = self.socket().read(buf.as_mut_slice())?;
        buf.resize(n, 0);
        let message = Message::decode(&buf)?;
        debug!("{:?}", message);
        Ok(message)
    }
}
