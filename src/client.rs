use connection::Connection;
use models::{Handshake, OpCode};
#[cfg(feature = "rich_presence")]
use rich_presence::{SetActivityArgs, SetActivity};
use error::Result;


#[derive(Debug)]
pub struct Client<T>
    where T: Connection
{
    client_id: u64,
    version: u32,
    socket: T,
}

impl<T> Client<T>
    where T: Connection
{
    pub fn new(client_id: u64) -> Result<Self> {
        let socket = T::connect()?;
        Ok(Self { version: 1, client_id, socket})
    }

    pub fn start(mut self) -> Result<Self> {
        self.handshake()?;
        Ok(self)
    }

    #[cfg(feature = "rich_presence")]
    pub fn set_activity<F>(&mut self, f: F) -> Result<()>
        where F: FnOnce(SetActivity) -> SetActivity
    {
        let args = SetActivityArgs::command(f(SetActivity::new()));
        self.socket.send(OpCode::Frame, args)?;
        Ok(())
    }

// private

    fn handshake(&mut self) -> Result<()> {
        let client_id = self.client_id;
        let version = self.version;
        self.socket.send(OpCode::Handshake, Handshake::new(client_id, version))?;
        Ok(())
    }
}
