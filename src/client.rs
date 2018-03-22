use std::env;
use std::io::{Write, Read, Result};
use std::os::unix::net::UnixStream;
use std::time;
use std::fmt::Debug;
use models::{Message, Handshake, Payload};
#[cfg(feature = "rich_presence")]
use models::{SetActivityArgs, SetActivity};

#[derive(Debug)]
pub struct Client {
    client_id: u64,
    version: u32,
    socket: UnixStream,
}

impl Client {
    pub fn new(client_id: u64) -> Result<Self> {
        let connection_name = Self::ipc_path();
        let socket = UnixStream::connect(connection_name)?;
        socket.set_write_timeout(Some(time::Duration::from_secs(30)))?;
        socket.set_read_timeout(Some(time::Duration::from_secs(30)))?;
        Ok(Self { version: 1, client_id, socket })
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
        self.send(1, args)?;
        Ok(())
    }

// private

    fn handshake(&mut self) -> Result<()> {
        let client_id = self.client_id;
        let version = self.version;
        self.send(0, Handshake::new(client_id, version))?;
        Ok(())
    }

    fn ipc_path() -> String {
        let tmp = env::var("XDG_RUNTIME_DIR").unwrap_or("/tmp".into());
        format!("{}/discord-ipc-0", tmp)
    }

    fn send<T>(&mut self, opcode: u32, payload: T) -> Result<()>
        where T: Payload + Debug
    {
        debug!("payload: {:#?}", payload);
        match Message::new(opcode, payload).encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket.write_all(bytes.as_ref())?;
                debug!("sent opcode: {}", opcode);
                self.receive()?;
            }
        };

        Ok(())
    }

    fn receive(&mut self) -> Result<()> {
        let mut buf: Vec<u8> = Vec::with_capacity(1024);
        self.socket.read(buf.as_mut_slice())?;
        debug!("{:?}", buf);
        Ok(())
    }
}
