use serde::{Serialize, de::DeserializeOwned};
#[allow(unused)]
use serde_json::Value;

use connection::Manager as ConnectionManager;
use models::{
    OpCode,
    Command,
    Event,
    payload::Payload,
    commands::{SubscriptionArgs, Subscription},
};
#[cfg(feature = "rich_presence")]
use models::rich_presence::{
    SetActivityArgs,
    Activity,
    SendActivityJoinInviteArgs,
    CloseActivityRequestArgs,
};
use error::{Result, Error};


pub struct Client {
    connection: ConnectionManager,
}

impl Client {
    pub fn new(client_id: u64) -> Result<Self> {
        Ok(Self {
            connection: ConnectionManager::new(client_id)?
        })
    }

    pub fn start(&mut self) {
        self.connection.start();
    }

    pub fn execute<A, E>(&mut self, cmd: Command, args: A, evt: Option<Event>) -> Result<Payload<E>>
        where A: Serialize + Send + Sync,
              E: Serialize + DeserializeOwned + Send + Sync
    {
        self.connection.send(OpCode::Frame, Payload::with_nonce(cmd, Some(args), None, evt))?;
        let (_, response): (OpCode, Payload<E>) = self.connection.recv()?;

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

    #[cfg(feature = "rich_presence")]
    pub fn clear_activity(&mut self) -> Result<Payload<Activity>> {
        self.execute(Command::SetActivity, SetActivityArgs::default(), None)
    }

    // NOTE: Not sure what the actual response values of
    //       SEND_ACTIVITY_JOIN_INVITE and CLOSE_ACTIVITY_REQUEST are,
    //       they are not documented.
    #[cfg(feature = "rich_presence")]
    pub fn send_activity_join_invite(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(Command::SendActivityJoinInvite, SendActivityJoinInviteArgs::new(user_id), None)
    }

    #[cfg(feature = "rich_presence")]
    pub fn close_activity_request(&mut self, user_id: u64) -> Result<Payload<Value>> {
        self.execute(Command::CloseActivityRequest, CloseActivityRequestArgs::new(user_id), None)
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
