use std::{
    io::{Write, Read},
    marker::Sized,
    fmt::Debug,
    path::PathBuf,
};

use models::{Payload, Message, OpCode};
use error::Result;


pub trait Connection
    where Self: Sized
{
    type Socket: Write + Read;


    fn socket(&mut self) -> &mut Self::Socket;

    fn ipc_path() -> PathBuf;

    fn connect() -> Result<Self>;

    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }

    fn send<T>(&mut self, opcode: OpCode, payload: T) -> Result<()>
        where T: Payload + Debug
    {
        debug!("payload: {:#?}", payload);
        match Message::new(opcode, payload).encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket().write_all(bytes.as_ref())?;
                debug!("sent opcode: {:?}", opcode);
                self.recv()?;
            }
        };
        Ok(())
    }

    fn recv(&mut self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let n = self.socket().read(buf.as_mut_slice())?;
        buf.resize(n, 0);
        debug!("{:?}", Message::decode(&buf));
        Ok(buf)
    }
}
