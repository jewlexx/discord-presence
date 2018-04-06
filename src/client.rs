use serde::{Serialize, de::DeserializeOwned};

use connection::{
    Connection,
    SocketConnection,
    Manager as ConnectionManager,
};
use models::{
    OpCode,
    Command,
    Event,
    payload::Payload,
    commands::{SubscriptionArgs, Subscription},
};

#[cfg(feature = "rich_presence")]
use models::rich_presence::{SetActivityArgs, Activity};
use error::{Result, Error};


pub struct Client<T>
    where T: Connection + Send + Sync + 'static
{
    connection: ConnectionManager<T>,
}

impl<T> Client<T>
    where T: Connection + Send + Sync + 'static
{
    pub fn with_connection(client_id: u64, connection: T) -> Result<Self> {
        Ok(Self {
            connection: ConnectionManager::with_connection(client_id, connection)?
        })
    }

    pub fn start(mut self) -> Result<Self> {
        self.connection.handshake()?;
        Ok(self)
    }

    pub fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
        where A: Serialize + Send + Sync,
              E: Serialize + DeserializeOwned + Send + Sync
    {
        self.connection.send(OpCode::Frame, Payload::with_nonce(cmd, Some(args), None, evt))?;
        let response: Payload<E> = self.connection.recv()?;

        match response.evt {
            Some(Event::Error) => Err(Error::SubscriptionFailed),
            _ => Ok(response)
        }
    }

    #[cfg(feature = "rich_presence")]
    pub fn set_activity<F>(&mut self, f: F) -> Result<Payload<Activity>>
        where F: FnOnce(Activity) -> Activity
    {
        self.execute(Command::SetActivity, SetActivityArgs::new(f), None)
    }

    pub fn subscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
        where F: FnOnce(SubscriptionArgs) -> SubscriptionArgs
    {
        self.execute(Command::Subscribe, f(SubscriptionArgs::new()), Some(evt))
    }

    pub fn unsubscribe<F>(&mut self, evt: Event, f: F) -> Result<Payload<Subscription>>
        where F: FnOnce(SubscriptionArgs) -> SubscriptionArgs
    {
        self.execute(Command::Unsubscribe, f(SubscriptionArgs::new()), Some(evt))
    }
}

impl Client<SocketConnection> {
    pub fn new(client_id: u64) -> Result<Self> {
        let socket = Connection::connect()?;
        Self::with_connection(client_id, socket)
    }
}
